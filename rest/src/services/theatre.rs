use chrono::{NaiveDate, NaiveDateTime};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use rayon::prelude::*;

use super::DatabaseError;
use crate::model::*;

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    x: f64,
    y: f64,
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

    pub async fn get_by_name(
        &self,
        name_: String,
    ) -> Result<Option<TheatreResource>, DatabaseError> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        let Some(theatre) = conn
            .interact(|conn| {
                theatres
                    .filter(name.eq(name_))
                    .limit(1)
                    .load::<Theatre>(conn)
            })
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

    pub async fn get_nearby(&self, location_: String, radius: f32) {
        todo!();
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

    pub async fn create_hall(&self, new_hall: FormHall) -> Result<HallResource, DatabaseError> {
        use crate::schema::halls::dsl::*;

        let conn = self.pool.get().await?;

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

        conn.interact(move |conn| {
            diesel::update(halls.filter(id.eq(id_)).filter(is_deleted.eq(false)))
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

        Ok(conn
            .interact(move |conn| {
                diesel::update(ticket_types.filter(id.eq(id_)).filter(is_deleted.eq(false)))
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

        conn.interact(move |conn| {
            diesel::update(ticket_types.filter(id.eq(id_)).filter(is_deleted.eq(false)))
                .set(is_deleted.eq(true))
                .execute(conn)
        })
        .await??;

        Ok(())
    }

    pub async fn query_tickets(
        &self,
        for_user: Option<uuid::Uuid>,
        for_theatre_movie: Option<uuid::Uuid>,
        ticket_type: Option<uuid::Uuid>,
        by_issuer: Option<uuid::Uuid>,
    ) -> Result<Vec<Ticket>, DatabaseError> {
        use crate::schema::tickets::dsl::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                let mut query = tickets.into_boxed();

                if let Some(for_user) = for_user {
                    query = query.filter(owner_user_id.eq(for_user));
                }

                if let Some(for_theatre_movie) = for_theatre_movie {
                    query = query.filter(theatre_screening_id.eq(for_theatre_movie));
                }

                if let Some(ticket_type) = ticket_type {
                    query = query.filter(ticket_type_id.eq(ticket_type));
                }

                if let Some(by_issuer) = by_issuer {
                    query = query.filter(issuer_user_id.eq(by_issuer));
                }

                query.load(conn)
            })
            .await??)
    }

    pub async fn query_screening_events(
        &self,
        start_date: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
    ) -> Result<Vec<TheatreScreeningEvent>, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;

        let mut query = theatre_screenings::table
            .inner_join(movies::table)
            .filter(theatre_screenings::is_deleted.eq(false))
            .filter(movies::is_deleted.eq(false))
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

        Ok(conn.interact(move |conn| query.load(conn)).await??)
    }

    pub async fn get_theatre_screening(
        &self,
        id_: uuid::Uuid,
    ) -> Result<Option<TheatreScreening>, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;

        let query = theatre_screenings::table.filter(theatre_screenings::id.eq(id_));

        Ok(conn
            .interact(move |conn| {
                query
                    .filter(theatre_screenings::is_deleted.eq(false))
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn create_theatre_screening(
        &self,
        theatre_screening: FormTheatreScreening,
    ) -> Result<TheatreScreening, DatabaseError> {
        let conn = self.pool.get().await?;

        Ok(conn
            .interact(|conn| {
                diesel::insert_into(crate::schema::theatre_screenings::table)
                    .values(theatre_screening)
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

        Ok(conn
            .interact(move |conn| {
                diesel::update(theatre_screenings.filter(id.eq(id_)).filter(is_deleted.eq(false)))
                    .set(new_theatre_screening)
                    .returning(TheatreScreening::as_returning())
                    .get_result(conn)
            })
            .await??)
    }

    pub async fn delete_theatre_screening(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::theatre_screenings::dsl::*;

        let conn = self.pool.get().await?;

        conn.interact(move |conn| {
            diesel::update(theatre_screenings.filter(id.eq(id_)).filter(is_deleted.eq(false)))
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
