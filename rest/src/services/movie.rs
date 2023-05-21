use deadpool_diesel::postgres::{Manager, Pool};
use diesel::prelude::*;

use crate::model::*;
use crate::password;

pub struct MovieService {
    pool: Pool,
}

impl MovieService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        movie: FormMovie,
    ) -> Result<Option<Movie>, Box<dyn std::error::Error>> {
        use crate::schema::movies::dsl::*;

        let conn = &mut self.pool.get().await?;

        Ok(conn
            .interact(move |conn| diesel::insert_into(movies).values(movie).load(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn get_by_id(&self, id_: i32) -> Result<Option<Movie>, Box<dyn std::error::Error>> {
        use crate::schema::movies::dsl::*;

        let conn = &mut self.pool.get().await?;

        Ok(conn
            .interact(move |conn| movies.filter(id.eq(id_)).limit(1).load::<Movie>(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn get_by_name(
        &self,
        name_: String,
    ) -> Result<Option<Movie>, Box<dyn std::error::Error>> {
        use crate::schema::movies::dsl::*;

        let conn = &mut self.pool.get().await?;

        Ok(conn
            .interact(|conn| movies.filter(name.eq(name_)).limit(1).load::<Movie>(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn delete(&self, id_: i32) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::movies::dsl::*;

        self.pool.get().await?.interact(move |conn| diesel::update(movies)
            .filter(id.eq(id_))
            .set(is_deleted.eq(true))
            .execute(conn)).await??;
        
        Ok(())
    }
}
