use actix_web::{
    body::BoxBody, delete, get, http::StatusCode, post, put, web, HttpResponse, Responder,
    ResponseError,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{
    openapi::{RefOr, Schema},
    PartialSchema, ToResponse, ToSchema,
};
use validator::{Validate, ValidationErrors};

use crate::{
    model::{JwtClaims, JwtType, User},
    services::{
        user::{UserResource, UserService},
        DatabaseError,
    },
};

pub mod auth;
pub mod language;
pub mod movie;
pub mod role;
pub mod theatre;
pub mod user;

#[derive(Serialize, Debug, ToSchema)]
#[serde(tag = "type", content = "data")]
pub enum ErrorType {
    #[schema(value_type = Object)]
    Validation(ValidationErrors),
    #[schema(value_type = Object)]
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

pub async fn user_res_from_jwt(
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
        .configure(theatre::config)
        .configure(movie::config)
        .configure(user::config)
        .configure(role::config)
        .configure(language::config);
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
        let res = json!(self.0);

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

impl ErrorType {
    fn status_code_diesel_error(err: &diesel::result::Error) -> actix_web::http::StatusCode {
        match err {
            diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn status_code_db_error(err: &DatabaseError) -> actix_web::http::StatusCode {
        match err {
            DatabaseError::Query(e) => ErrorType::status_code_diesel_error(e),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ResponseError for ErrorType {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let res = json!({
            "error": &self,
        });

        HttpResponse::build(self.status_code()).json(res)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ErrorType::Database(e) => ErrorType::status_code_db_error(e),
            ErrorType::Validation(_) | ErrorType::Invalid => StatusCode::BAD_REQUEST,
            ErrorType::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::InsufficientPermission => StatusCode::FORBIDDEN,
            ErrorType::Expired | ErrorType::NoAuth | ErrorType::EmailNotVerified => {
                StatusCode::UNAUTHORIZED
            }
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::Conflict => StatusCode::CONFLICT,
        }
    }
}

#[derive(ToSchema, Serialize)]
pub struct DocError {
    pub error: ErrorType,
}

#[macro_export]
macro_rules! doc {
    ($err:expr) => {
        DocError { error: $err }
    };
}
