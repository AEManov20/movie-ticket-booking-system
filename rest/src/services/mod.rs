pub mod movie;
pub mod theatre;
pub mod user;
pub mod bridge_role;
pub mod role;
pub mod language;

use argon2::password_hash;
use deadpool_diesel::{InteractError, PoolError};
use either::Either;
use lettre::{address::AddressError, Message};
use serde::Deserialize;
use tokio::sync::mpsc::error::SendError;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Copy, Clone, ToSchema)]
pub enum SortBy {
    Newest,
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
    #[error("something went wrong when sending an email")]
    EmailSend(#[from] SendError<Message>),
    #[error("something went wrong when building an email")]
    EmailBuild(Either<AddressError, lettre::error::Error>),
    #[error("{}", .0)]
    Other(String)
}

impl From<AddressError> for DatabaseError {
    fn from(value: AddressError) -> Self {
        Self::EmailBuild(Either::Left(value))
    }
}

impl From<lettre::error::Error> for DatabaseError {
    fn from(value: lettre::error::Error) -> Self {
        Self::EmailBuild(Either::Right(value))
    }
}

impl From<password_hash::Error> for DatabaseError {
    fn from(value: password_hash::Error) -> Self {
        Self::Hash(value)
    }
}
