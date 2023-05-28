use deadpool_diesel::postgres::{Manager, Pool};
use diesel::prelude::*;
use serde::Deserialize;

use crate::model::*;
use crate::password;

#[derive(Clone)]
pub struct MovieService {
    pool: Pool,
}

#[derive(Deserialize, Copy, Clone)]
pub enum SortBy {
    #[serde(alias = "newest")]
    Newest,
    #[serde(alias = "oldest")]
    Oldest,
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

    pub async fn get_by_id(&self, id_: uuid::Uuid) -> Result<Option<Movie>, Box<dyn std::error::Error>> {
        use crate::schema::movies::dsl::*;

        let conn = &mut self.pool.get().await?;

        Ok(conn
            .interact(move |conn| movies.filter(id.eq(id_)).limit(1).load::<Movie>(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn query_movies(
        &self,
        name_: String,
        limit: i64,
        offset: i64,
        sort_by: SortBy,
    ) -> Result<Vec<Movie>, Box<dyn std::error::Error>> {
        use crate::schema::movies::dsl::*;

        let conn = &mut self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                let query = movies.limit(limit).offset(offset).filter(name.ilike(name_));

                match sort_by {
                    SortBy::Newest => query.order_by(release_date.desc()).load::<Movie>(conn),
                    SortBy::Oldest => query.order_by(release_date.asc()).load::<Movie>(conn),
                }
            })
            .await??)
    }

    pub async fn delete(&self, id_: uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::movies::dsl::*;

        self.pool
            .get()
            .await?
            .interact(move |conn| {
                diesel::update(movies)
                    .filter(id.eq(id_))
                    .set(is_deleted.eq(true))
                    .execute(conn)
            })
            .await??;

        Ok(())
    }
}
