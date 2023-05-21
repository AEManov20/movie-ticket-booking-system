use deadpool_diesel::postgres::{Manager, Pool};
use diesel::prelude::*;

use crate::model::*;
use crate::password;

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

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
    ) -> Result<Option<Theatre>, Box<dyn std::error::Error>> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(|conn| diesel::insert_into(theatres).values(theatre).load(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn update(
        &self,
        id_: i32,
        theatre: FormTheatre,
    ) -> Result<Option<Theatre>, Box<dyn std::error::Error>> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                diesel::update(theatres)
                    .filter(id.eq(id_))
                    .set(theatre)
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn get_by_name(
        &self,
        name_: String,
    ) -> Result<Option<Theatre>, Box<dyn std::error::Error>> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(|conn| {
                theatres
                    .filter(name.eq(name_))
                    .limit(1)
                    .load::<Theatre>(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn get_by_id(&self, id_: i32) -> Result<Option<Theatre>, Box<dyn std::error::Error>> {
        use crate::schema::theatres::dsl::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| theatres.filter(id.eq(id_)).limit(1).load::<Theatre>(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn get_nearby(&self, location_: String, radius: f32) {
        todo!();
    }

    pub async fn delete(&self, id_: i32) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::theatres::dsl::*;

        self.pool.get().await?.interact(move |conn| diesel::update(theatres)
            .filter(id.eq(id_))
            .set(is_deleted.eq(true))
            .execute(conn)).await??;

        Ok(())
    }
}

pub struct TheatreResource {
    theatre: Theatre,
    pool: Pool
}

impl TheatreResource {
    
}
