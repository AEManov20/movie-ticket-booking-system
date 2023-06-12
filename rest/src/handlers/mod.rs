use actix_web::{
    body::BoxBody, delete, get, http::StatusCode, post, put, web, HttpResponse, Responder,
    ResponseError,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidationErrors};

use crate::{
    model::{JwtClaims, JwtType, User},
    services::{
        user::{UserResource, UserService},
        DatabaseError,
    },
};

pub mod auth;
pub mod movie;
pub mod role;
pub mod theatre;
pub mod user;

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ErrorType {
    Validation(ValidationErrors),
    Database(#[serde(skip)] DatabaseError),
    InsufficientPermission,
    EmailNotVerified,
    ServerError,
    Conflict,
    Invalid,
    NoAuth,
    NotFound,
    Expired,
}

pub struct SuccessResponse<T>(pub T);

type Result<T> = std::result::Result<SuccessResponse<T>, ErrorType>;

async fn user_res_from_jwt(
    claims: &JwtClaims,
    user_service: &UserService,
) -> std::result::Result<(UserResource, User), ErrorType> {
    let JwtType::User(user_id) = claims.dat else {
        return Err(ErrorType::NoAuth)
    };

    let user = user_service.get_by_id(user_id).await?;

    match user {
        Some(v) => Ok((v.clone(), User::from(v))),
        None => Err(ErrorType::Database(DatabaseError::Other(
            "Database returned nothing.".to_string(),
        ))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::config)
        .configure(movie::config)
        .configure(theatre::config)
        .configure(user::config)
        .configure(role::config);
}

impl<T> From<T> for SuccessResponse<T>
where
    T: Serialize,
{
    fn from(value: T) -> Self {
        SuccessResponse(value)
    }
}

impl<T> Responder for SuccessResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let res = json!({
            "success": true,
            "data": self.0
        });

        HttpResponse::Ok().json(res)
    }
}

impl From<DatabaseError> for ErrorType {
    fn from(value: DatabaseError) -> Self {
        ErrorType::Database(value)
    }
}

impl From<ValidationErrors> for ErrorType {
    fn from(value: ValidationErrors) -> Self {
        ErrorType::Validation(value)
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ErrorType {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let res = json!({
            "success": false,
            "error": &self,
        });

        HttpResponse::build(self.status_code()).json(res)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ErrorType::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::Validation(_) => StatusCode::BAD_REQUEST,
            ErrorType::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::InsufficientPermission => StatusCode::FORBIDDEN,
            ErrorType::EmailNotVerified => StatusCode::UNAUTHORIZED,
            ErrorType::Invalid => StatusCode::BAD_REQUEST,
            ErrorType::Expired => StatusCode::UNAUTHORIZED,
            ErrorType::NoAuth => StatusCode::UNAUTHORIZED,
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::Conflict => StatusCode::CONFLICT,
        }
    }
}
