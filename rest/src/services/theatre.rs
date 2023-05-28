use deadpool_diesel::postgres::{Manager, Pool};
use diesel::prelude::*;

use crate::model::*;
use crate::password;
use crate::schema::tickets::owner_user_id;

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

    pub async fn create(
        &self,
        theatre: FormTheatre,
    ) -> Result<Option<TheatreResource>, Box<dyn std::error::Error>> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        let Some(theatre) = conn
            .interact(|conn| diesel::insert_into(theatres).values(theatre).load::<Theatre>(conn))
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

    pub async fn update(
        &self,
        id_: uuid::Uuid,
        theatre: FormTheatre,
    ) -> Result<Option<TheatreResource>, Box<dyn std::error::Error>> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        let Some(theatre) = conn
            .interact(move |conn| {
                diesel::update(theatres)
                    .filter(id.eq(id_))
                    .set(theatre)
                    .load::<Theatre>(conn)
            })
            .await??
            .first()
            .cloned() else {
                return Ok(None);
            };

        Ok(Some(TheatreResource {
            theatre,
            pool: self.pool.clone(),
        }))
    }

    pub async fn get_by_name(
        &self,
        name_: String,
    ) -> Result<Option<TheatreResource>, Box<dyn std::error::Error>> {
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

    pub async fn get_by_id(&self, id_: uuid::Uuid) -> Result<Option<TheatreResource>, Box<dyn std::error::Error>> {
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

    pub async fn delete(&self, id_: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::theatres::dsl::*;

        self.pool
            .get()
            .await?
            .interact(move |conn| {
                diesel::update(theatres)
                    .filter(id.eq(id_))
                    .set(is_deleted.eq(true))
                    .execute(conn)
            })
            .await??;

        Ok(())
    }
}

pub struct TheatreResource {
    theatre: Theatre,
    pool: Pool,
}

impl TheatreResource {
    async fn get_halls(&self) -> Result<Vec<HallResource>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| Hall::belonging_to(&theatre).load::<Hall>(conn))
            .await??
            .iter()
            .map(|el| HallResource {
                hall: el.clone(),
                pool: self.pool.clone(),
            })
            .collect())
    }

    async fn get_permissions(&self) -> Result<Vec<TheatrePermission>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| TheatrePermission::belonging_to(&theatre).load(conn))
            .await??)
    }

    async fn get_ticket_types(&self) -> Result<Vec<TicketType>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let theatre = self.theatre.clone();

        Ok(conn
            .interact(move |conn| TicketType::belonging_to(&theatre).load(conn))
            .await??)
    }

    async fn update_ticket_type(
        &self,
        id_: uuid::Uuid,
        new_ticket_type: FormTicketType,
    ) -> Result<Option<TicketType>, Box<dyn std::error::Error>> {
        use crate::schema::ticket_types::dsl::*;
        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                diesel::update(ticket_types)
                    .filter(id.eq(id_))
                    .set(&new_ticket_type)
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    async fn create_ticket_type(
        &self,
        new_ticket_type: FormTicketType,
    ) -> Result<Option<TicketType>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                diesel::insert_into(crate::schema::ticket_types::table)
                    .values(&new_ticket_type)
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    async fn delete_ticket_type(&self, id_: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::ticket_types::dsl::*;

        let conn = self.pool.get().await?;

        conn.interact(move |conn| {
            diesel::delete(ticket_types)
                .filter(id.eq(id_))
                .execute(conn)
        })
        .await??;

        Ok(())
    }

    async fn query_tickets(
        &self,
        for_user: Option<uuid::Uuid>,
        for_theatre_movie: Option<uuid::Uuid>,
        ticket_type: Option<uuid::Uuid>,
        by_issuer: Option<uuid::Uuid>,
    ) -> Result<Vec<Ticket>, Box<dyn std::error::Error>> {
        use crate::schema::tickets::dsl::*;

        let mut conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                let mut query = tickets.into_boxed();

                if let Some(for_user) = for_user {
                    query = query.filter(owner_user_id.eq(for_user));
                }

                if let Some(for_theatre_movie) = for_theatre_movie {
                    query = query.filter(theatre_movie_id.eq(for_theatre_movie));
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
}

pub struct HallResource {
    hall: Hall,
    pool: Pool,
}

impl HallResource {
    async fn get_theatre_movies(&self) -> Result<Vec<TheatreMovie>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let hall = self.hall.clone();

        Ok(conn
            .interact(move |conn| TheatreMovie::belonging_to(&hall).load(conn))
            .await??)
    }

    async fn new_theatre_movie(
        &self,
        theatre_movie: FormTheatreMovie,
    ) -> Result<Option<TheatreMovie>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        Ok(conn
            .interact(|conn| {
                diesel::insert_into(crate::schema::theatre_movies::table)
                    .values(theatre_movie)
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    async fn update_theatre_movie(
        &self,
        id_: uuid::Uuid,
        new_theatre_movie: FormTheatreMovie,
    ) -> Result<Option<TheatreMovie>, Box<dyn std::error::Error>> {
        use crate::schema::theatre_movies::dsl::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                diesel::update(theatre_movies)
                    .filter(id.eq(id_))
                    .set(new_theatre_movie)
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    async fn delete_theatre_movie(&self, id_: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::theatre_movies::dsl::*;

        let conn = self.pool.get().await?;

        conn.interact(move |conn| {
            diesel::delete(theatre_movies)
                .filter(id.eq(id_))
                .execute(conn)
        })
        .await??;
        Ok(())
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
