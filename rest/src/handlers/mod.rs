use actix_web::{delete, get, http::{header, StatusCode}, post, put, web, HttpResponse, Responder, ResponseError};
use log::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use std::fmt;

use crate::{services::{user::{UserService, UserResource}, self}, model::{JwtClaims, JwtType}};

pub mod auth;
pub mod movie;
pub mod theatre;
pub mod user;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum ErrorType {
    Validation(ValidationErrors),
    Database(String),
    ServerError,
    Conflict,
    Invalid,
    Expired,
    NotFound,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse<T> {
    pub error: T,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum FetchUserErrorType {
    DatabaseError,
    UserDoesNotExist,
    InvalidToken,
}

async fn user_res_from_jwt(claims: &JwtClaims, user_service: &UserService) -> Result<UserResource, ErrorResponse<FetchUserErrorType>> {
    if let JwtType::User(user_id) = claims.dat {
        match user_service.get_by_id(user_id).await {
            Ok(v) => {
                match v {
                    Some(v) => Ok(v),
                    None => Err(ErrorResponse { error: FetchUserErrorType::UserDoesNotExist })
                }
            },
            Err(e) => {
                Err(ErrorResponse { error: FetchUserErrorType::DatabaseError })
            }
        }
    } else {
        Err(ErrorResponse { error: FetchUserErrorType::InvalidToken })
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::config)
        .configure(movie::config)
        .configure(theatre::config)
        .configure(user::config);
}

// impl ResponseError for services::DatabaseError {
//     fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
//         HttpResponse::build(self.status_code())
//             .json(value)
//     }

//     fn status_code(&self) -> actix_web::http::StatusCode {
//         StatusCode::INTERNAL_SERVER_ERROR
//     }
// }

impl<T> fmt::Display for ErrorResponse<T>
where T:
    Serialize
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}