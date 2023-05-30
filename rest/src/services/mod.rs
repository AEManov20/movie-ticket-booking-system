pub mod movie;
pub mod theatre;
pub mod user;

use argon2::password_hash;
use deadpool_diesel::{InteractError, PoolError};
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone)]
pub enum SortBy {
    #[serde(alias = "newest")]
    Newest,
    #[serde(alias = "oldest")]
    Oldest,
}

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("couldn't fetch pool")]
    Pool(#[from] PoolError),
    #[error("hashing was unsuccessful")]
    Hash(password_hash::Error),
    #[error("jwt token creation was unsuccessful")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("interaction with database unsuccessful")]
    Interact(#[from] InteractError),
    #[error("query did not execute properly")]
    Query(#[from] diesel::result::Error),
    #[error("{}", .0)]
    Other(String)
}

impl From<password_hash::Error> for DatabaseError {
    fn from(value: password_hash::Error) -> Self {
        Self::Hash(value)
    }
}
