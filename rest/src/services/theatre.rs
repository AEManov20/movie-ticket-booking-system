use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use deadpool_diesel::postgres::Pool;
use diesel::{dsl::count_distinct, pg::Pg, prelude::*};
use rayon::prelude::*;

use super::DatabaseError;
use crate::schema::*;
use crate::{model::*, services::user::TicketResource};

macro_rules! theatres_with_counts {
    () => {
        theatres::table
            .left_join(halls::table)
            .left_join(theatre_screenings::table)
            .left_join(tickets::table.on(theatre_screenings::id.eq(tickets::theatre_screening_id)))
            .group_by(theatres::id)
            .having(count_distinct(halls::id.nullable()).gt(0))
            .select((
                theatres::id,
                theatres::name,
                theatres::location_lat,
                theatres::location_lon,
                theatres::logo_image_url,
                theatres::cover_image_url,
                count_distinct(theatre_screenings::id.nullable()),
                count_distinct(halls::id.nullable()),
                count_distinct(tickets::id.nullable()),
            ))
    };
}

#[derive(Clone)]
pub struct TheatreService {
    pool: Pool,
}

impl TheatreService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, theatre: FormTheatre) -> Result<TheatreResource, DatabaseError> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        let theatre = conn
            .interact(|conn| {
                diesel::insert_into(theatres)
                    .values(theatre)
                    .returning(Theatre::as_returning())
                    .get_result(conn)
            })
            .await??;

        Ok(TheatreResource {
            theatre,
            pool: self.pool.clone(),
        })
    }

    pub async fn update(
        &self,
        id_: uuid::Uuid,
        theatre: FormTheatre,
    ) -> Result<TheatreResource, DatabaseError> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        let theatre = conn
            .interact(move |conn| {
                diesel::update(theatres.filter(id.eq(id_)).filter(is_deleted.eq(false)))
                    .set(theatre)
                    .returning(Theatre::as_returning())
                    .get_result(conn)
            })
            .await??;

        Ok(TheatreResource {
            theatre,
            pool: self.pool.clone(),
        })
    }

    pub async fn get_by_name_extra(
        &self,
        name: String,
    ) -> Result<Vec<ExtendedTheatre>, DatabaseError> {
        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                theatres_with_counts!()
                    .filter(theatres::name.ilike(format!("%{name}%")))
                    .load(conn)
            })
            .await??)
    }

    pub async fn get_by_id(
        &self,
        id_: uuid::Uuid,
    ) -> Result<Option<TheatreResource>, DatabaseError> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        let Some(theatre) = conn
            .interact(move |conn| theatres.filter(id.eq(id_)).limit(1).load::<Theatre>(conn))
            .await??
            .first()
            .cloned() else {
                return Ok(None)
            };

        Ok(Some(TheatreResource {
            theatre,
            pool: self.pool.clone(),
        }))
    }

    pub async fn get_by_id_extra(
        &self,
        id_: uuid::Uuid,
    ) -> Result<Option<ExtendedTheatre>, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                theatres_with_counts!()
                    .filter(theatres::id.eq(id_))
                    .load::<ExtendedTheatre>(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn get_nearby_extended(
        &self,
        location: Point,
    ) -> Result<Vec<ExtendedTheatre>, DatabaseError> {
        const DISTANCE: f64 = 16.;
        use crate::schema::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                theatres_with_counts!()
                    .filter(
                        ((theatres::location_lat - location.y)
                            * (theatres::location_lat - location.y)
                            + (theatres::location_lon - location.x)
                                * (theatres::location_lon - location.x))
                            .lt(DISTANCE),
                    )
                    .load(conn)
            })
            .await??)
    }

    pub async fn delete(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::theatres::dsl::*;

        self.pool
            .get()
            .await?
            .interact(move |conn| {
                diesel::update(theatres.filter(id.eq(id_)).filter(is_deleted.eq(false)))
                    .set(is_deleted.eq(true))
                    .execute(conn)
            })
            .await??;

        Ok(())
    }
}

#[derive(Clone)]
pub struct TheatreResource {
    theatre: Theatre,
    pool: Pool,
}

impl TheatreResource {
    pub async fn get_halls(&self) -> Result<Vec<HallResource>, DatabaseError> {
        let conn = self.pool.get().await?;

        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| {
                Hall::belonging_to(&theatre)
                    .filter(crate::schema::halls::is_deleted.eq(false))
                    .load::<Hall>(conn)
            })
            .await??
            .par_iter()
            .map(|el| HallResource {
                hall: el.clone(),
                pool: self.pool.clone(),
            })
            .collect())
    }

    pub async fn hall_id_belongs(&self, hid: uuid::Uuid) -> Result<bool, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;
        let theatre = self.theatre.clone();

        match conn
            .interact(move |conn| {
                Hall::belonging_to(&theatre)
                    .filter(halls::is_deleted.eq(false))
                    .filter(halls::id.eq(hid))
                    .load::<Hall>(conn)
            })
            .await??
            .first()
            .cloned()
        {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn create_hall(&self, new_hall: FormHall) -> Result<HallResource, DatabaseError> {
        use crate::schema::halls::dsl::*;

        let conn = self.pool.get().await?;
        let new_hall = CreateHall::from_form(new_hall, self.theatre.id);

        let new_hall: Hall = conn
            .interact(move |conn| {
                diesel::insert_into(halls)
                    .values(new_hall)
                    .returning(Hall::as_returning())
                    .get_result(conn)
            })
            .await??;

        Ok(HallResource::new(new_hall, self.pool.clone()))
    }

    pub async fn delete_hall(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::halls::dsl::*;

        let conn = self.pool.get().await?;
        let theatre = self.theatre.clone();

        conn.interact(move |conn| {
            diesel::update(
                halls
                    .filter(id.eq(id_))
                    .filter(theatre_id.eq(theatre.id))
                    .filter(is_deleted.eq(false)),
            )
            .set(is_deleted.eq(true))
            .execute(conn)
        })
        .await??;

        Ok(())
    }

    pub async fn get_ticket_types(&self) -> Result<Vec<TicketType>, DatabaseError> {
        let conn = self.pool.get().await?;

        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| {
                TicketType::belonging_to(&theatre)
                    .filter(crate::schema::ticket_types::is_deleted.eq(false))
                    .load(conn)
            })
            .await??)
    }

    pub async fn update_ticket_type(
        &self,
        id_: uuid::Uuid,
        new_ticket_type: FormTicketType,
    ) -> Result<TicketType, DatabaseError> {
        use crate::schema::ticket_types::dsl::*;
        let conn = self.pool.get().await?;
        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| {
                diesel::update(
                    ticket_types
                        .filter(id.eq(id_))
                        .filter(theatre_id.eq(theatre.id))
                        .filter(is_deleted.eq(false)),
                )
                .set(&new_ticket_type)
                .returning(TicketType::as_returning())
                .get_result(conn)
            })
            .await??)
    }

    pub async fn create_ticket_type(
        &self,
        new_ticket_type: FormTicketType,
    ) -> Result<TicketType, DatabaseError> {
        let conn = self.pool.get().await?;
        let new_ticket_type = CreateTicketType::from_form(new_ticket_type, self.theatre.id);

        Ok(conn
            .interact(move |conn| {
                diesel::insert_into(crate::schema::ticket_types::table)
                    .values(&new_ticket_type)
                    .returning(TicketType::as_returning())
                    .get_result(conn)
            })
            .await??)
    }

    pub async fn delete_ticket_type(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::ticket_types::dsl::*;

        let conn = self.pool.get().await?;
        let theatre = self.theatre.clone();

        conn.interact(move |conn| {
            diesel::update(
                ticket_types
                    .filter(id.eq(id_))
                    .filter(theatre_id.eq(theatre.id))
                    .filter(is_deleted.eq(false)),
            )
            .set(is_deleted.eq(true))
            .execute(conn)
        })
        .await??;

        Ok(())
    }

    pub async fn get_ticket_by_id(
        &self,
        tid: uuid::Uuid,
    ) -> Result<Option<TicketResource>, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;
        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| {
                tickets::table
                    .inner_join(theatre_screenings::table)
                    .filter(theatre_screenings::theatre_id.eq(theatre.id))
                    .filter(tickets::id.eq(tid))
                    .select(Ticket::as_select())
                    .load(conn)
            })
            .await??
            .first()
            .cloned()
            .map(|x| TicketResource::new(x, self.pool.clone())))
    }

    pub async fn query_tickets(&self, tquery: TicketQuery) -> Result<Vec<Ticket>, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;
        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| {
                let mut query = tickets::table
                    .inner_join(theatre_screenings::table)
                    .filter(theatre_screenings::theatre_id.eq(theatre.id))
                    .limit(tquery.limit)
                    .offset(tquery.offset)
                    .select(Ticket::as_select())
                    .into_boxed();

                if let Some(owner_id) = tquery.owner_id {
                    query = query.filter(tickets::owner_user_id.eq(owner_id));
                }

                if let Some(screening_id) = tquery.screening_id {
                    query = query.filter(tickets::theatre_screening_id.eq(screening_id));
                }

                if let Some(ticket_type_id) = tquery.ticket_type_id {
                    query = query.filter(tickets::ticket_type_id.eq(ticket_type_id));
                }

                if let Some(issuer_id) = tquery.issuer_id {
                    query = query.filter(tickets::issuer_user_id.eq(issuer_id));
                }

                if let Some(hall_id) = tquery.hall_id {
                    query = query.filter(theatre_screenings::hall_id.eq(hall_id));
                }

                if let Some(movie_id) = tquery.movie_id {
                    query = query.filter(theatre_screenings::movie_id.eq(movie_id));
                }

                query.load(conn)
            })
            .await??)
    }

    pub async fn query_screening_events(
        &self,
        start_date: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
        hall_id: Option<uuid::Uuid>
    ) -> Result<Vec<TheatreScreeningEvent>, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;

        let mut query = theatre_screenings::table
            .inner_join(movies::table)
            .inner_join(halls::table)
            .filter(theatre_screenings::is_deleted.eq(false))
            .filter(movies::is_deleted.eq(false))
            .filter(theatre_screenings::theatre_id.eq(self.theatre.id))
            .select((
                movies::id,
                theatre_screenings::id,
                theatre_screenings::starting_time,
                movies::length,
                movies::name,
            ))
            .filter(theatre_screenings::starting_time.gt(start_date))
            .into_boxed();

        if let Some(end_date) = end_date {
            query = query.filter(theatre_screenings::starting_time.lt(end_date))
        }

        if let Some(hall_id) = hall_id {
            query = query.filter(halls::id.eq(hall_id))
        }

        Ok(conn.interact(move |conn| query.load(conn)).await??)
    }

    pub async fn get_theatre_screening(
        &self,
        id_: uuid::Uuid,
    ) -> Result<Option<TheatreScreening>, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;

        let query = theatre_screenings::table
            .filter(theatre_screenings::theatre_id.eq(self.theatre.id))
            .filter(theatre_screenings::id.eq(id_))
            .filter(theatre_screenings::is_deleted.eq(false));

        Ok(conn
            .interact(move |conn| query.load(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn create_theatre_screening(
        &self,
        new_theatre_screening: FormTheatreScreening,
    ) -> Result<TheatreScreening, DatabaseError> {
        let conn = self.pool.get().await?;
        let new_theatre_screening =
            CreateTheatreScreening::from_form(new_theatre_screening, self.theatre.id);

        Ok(conn
            .interact(|conn| {
                diesel::insert_into(crate::schema::theatre_screenings::table)
                    .values(new_theatre_screening)
                    .returning(TheatreScreening::as_returning())
                    .get_result(conn)
            })
            .await??)
    }

    pub async fn update_theatre_screening(
        &self,
        id_: uuid::Uuid,
        new_theatre_screening: FormTheatreScreening,
    ) -> Result<TheatreScreening, DatabaseError> {
        use crate::schema::theatre_screenings::dsl::*;

        let conn = self.pool.get().await?;
        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| {
                diesel::update(
                    theatre_screenings
                        .filter(theatre_id.eq(theatre.id))
                        .filter(id.eq(id_))
                        .filter(is_deleted.eq(false)),
                )
                .set(new_theatre_screening)
                .returning(TheatreScreening::as_returning())
                .get_result(conn)
            })
            .await??)
    }

    pub async fn delete_theatre_screening(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::theatre_screenings::dsl::*;

        let conn = self.pool.get().await?;
        let theatre = self.theatre.clone();

        conn.interact(move |conn| {
            diesel::update(
                theatre_screenings
                    .filter(id.eq(id_))
                    .filter(theatre_id.eq(theatre.id))
                    .filter(is_deleted.eq(false)),
            )
            .set(is_deleted.eq(true))
            .execute(conn)
        })
        .await??;
        Ok(())
    }
}

#[derive(Clone)]
pub struct HallResource {
    hall: Hall,
    pool: Pool,
}

impl HallResource {
    fn new(hall: Hall, pool: Pool) -> Self {
        Self { hall, pool }
    }

    pub async fn get_theatre_movies(&self) -> Result<Vec<TheatreScreening>, DatabaseError> {
        let conn = self.pool.get().await?;

        let hall = self.hall.clone();

        Ok(conn
            .interact(move |conn| TheatreScreening::belonging_to(&hall).load(conn))
            .await??)
    }
}

impl From<TheatreResource> for Theatre {
    fn from(value: TheatreResource) -> Self {
        Self { ..value.theatre }
    }
}

impl From<HallResource> for Hall {
    fn from(value: HallResource) -> Self {
        Self { ..value.hall }
    }
}
