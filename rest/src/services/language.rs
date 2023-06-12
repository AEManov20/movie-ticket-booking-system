use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use crate::model::Language;

use super::*;

#[derive(Clone)]
pub struct LanguageService {
    pool: Pool
}

impl LanguageService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// fetches a `Language` by id
    pub async fn get_language_by_id(&self, lid: uuid::Uuid) -> Result<Option<Language>, DatabaseError> {
        use crate::schema::languages::dsl::*;

        Ok(self
            .pool
            .get()
            .await?
            .interact(move |conn| {
                languages
                    .filter(id.eq(lid))
                    .limit(1)
                    .load::<Language>(conn)
            })
            .await??
            .first()
            .cloned())
    }

    /// fetches a `Language` by name
    pub async fn get_language_by_name(&self, nm: String) -> Result<Option<Language>, DatabaseError> {
        use crate::schema::languages::dsl::*;

        Ok(self
            .pool
            .get()
            .await?
            .interact(move |conn| languages.filter(name.eq(nm.to_string())).limit(1).load(conn))
            .await??
            .first()
            .cloned())
    }

    /// fetches a `Vec` of `Language`
    pub async fn get_all_languages(&self) -> Result<Vec<Language>, DatabaseError> {
        use crate::schema::languages::dsl::*;

        Ok(self
            .pool
            .get()
            .await?
            .interact(move |conn| languages.load(conn))
            .await??)
    }
}