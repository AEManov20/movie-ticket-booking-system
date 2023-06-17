use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::model::*;

use super::DatabaseError;
use super::SortBy;

#[derive(Clone)]
pub struct MovieService {
    pool: Pool,
}

impl MovieService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, movie: FormMovie) -> Result<Movie, DatabaseError> {
        use crate::schema::movies::dsl::*;

        let conn = &mut self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                diesel::insert_into(movies)
                    .values(movie)
                    .returning(Movie::as_returning())
                    .get_result(conn)
            })
            .await??)
    }

    pub async fn get_by_id(&self, id_: uuid::Uuid) -> Result<Option<Movie>, DatabaseError> {
        use crate::schema::movies::dsl::*;

        let conn = &mut self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                movies
                    .filter(is_deleted.eq(false))
                    .filter(id.eq(id_))
                    .limit(1)
                    .load::<Movie>(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn query_movies(
        &self,
        name_: Option<String>,
        limit: i64,
        offset: i64,
        sort_by: SortBy,
    ) -> Result<Vec<Movie>, DatabaseError> {
        use crate::schema::movies::dsl::*;

        let conn = &mut self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                let mut query = movies
                    .filter(is_deleted.eq(false))
                    .limit(limit)
                    .offset(offset)
                    .into_boxed();

                if let Some(name_) = name_ {
                    query = query.filter(name.like(name_));
                }

                query = match sort_by {
                    SortBy::Newest => query.order_by(release_date.desc()),
                    SortBy::Oldest => query.order_by(release_date.asc()),
                };

                query.load(conn)
            })
            .await??)
    }

    pub async fn delete(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::movies::dsl::*;

        self.pool
            .get()
            .await?
            .interact(move |conn| {
                diesel::update(movies.filter(id.eq(id_)).filter(is_deleted.eq(false)))
                    .set(is_deleted.eq(true))
                    .execute(conn)
            })
            .await??;

        Ok(())
    }

    pub async fn get_review_by_id(
        &self,
        id_: uuid::Uuid,
    ) -> Result<Option<MovieReview>, DatabaseError> {
        use crate::schema::movie_reviews::dsl::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| movie_reviews.filter(id.eq(id_)).load(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn delete_review_by_id(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::movie_reviews::dsl::*;

        let conn = self.pool.get().await?;

        conn.interact(move |conn| {
            diesel::delete(movie_reviews)
                .filter(id.eq(id_))
                .execute(conn)
        })
        .await??;

        Ok(())
    }

    pub async fn query_reviews(
        &self,
        movie_id_: uuid::Uuid,
        limit: i64,
        offset: i64,
        sort_by: SortBy,
    ) -> Result<Vec<MovieReview>, DatabaseError> {
        use crate::schema::movie_reviews::dsl::*;

        let conn = self.pool.get().await?;

        Ok(conn
            .interact(move |conn| {
                let query = movie_reviews
                    .filter(movie_id.eq(movie_id_))
                    .limit(limit)
                    .offset(offset);

                match sort_by {
                    SortBy::Newest => query.order_by(created_at.desc()).load::<MovieReview>(conn),
                    SortBy::Oldest => query.order_by(created_at.asc()).load::<MovieReview>(conn),
                }
            })
            .await??)
    }
}
