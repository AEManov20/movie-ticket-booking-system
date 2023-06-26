use std::{str::FromStr, sync::Arc};

use super::*;
use chrono::Utc;
use lettre::{
    message::{header::ContentType, Mailbox},
    Address, Message, SmtpTransport, Transport,
};
use tokio::sync::Mutex;

use super::ErrorType;
use crate::{
    doc,
    model::{FormUser, JwtType, LoginUser, User},
    services::user::{LoginResponse, UserResource, UserService},
    vars::gmail_user, mailer::Mailer,
};

use utoipa::{IntoParams, ToSchema};

// TODO: implement auth from other providers

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct EmailVerificationQuery {
    pub email_key: String,
}

/// Returns an auth token, given correct login data is supplied
#[utoipa::path(
    context_path = "/api/v1/auth",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = UNAUTHORIZED, description = "Email is not verified"),
        (status = CONFLICT, description = "User isn't registered with a password but rather with an external provider"),
        (status = OK, description = "User successfully logged in and auth key returned", body = LoginResponse)
    ),
    params(
        LoginUser
    )
)]
#[get("/login")]
pub async fn login_user(
    params: web::Query<LoginUser>,
    user_service: web::Data<UserService>,
) -> Result<LoginResponse> {
    params.validate()?;

    let Some(user_res) = user_service.get_by_email_or_username(params.email.clone(), params.email.clone()).await? else {
        return Err(ErrorType::Invalid)
    };

    let user = User::from(user_res.clone());

    if !user.is_activated {
        return Err(ErrorType::EmailNotVerified);
    }

    let Some(password_hash) = user.password_hash else {
        return Err(ErrorType::Conflict)
    };

    if crate::password::verify(params.password.as_bytes(), &password_hash) {
        Ok(user_res.create_jwt()?.into())
    } else {
        Err(ErrorType::Invalid)
    }
}

/// Registers a new user given that the supplied data is valid
#[utoipa::path(
    context_path = "/api/v1/auth",
    request_body = FormUser,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = CONFLICT, description = "User already registered"),
        (status = OK, description = "User successfully registered")
    )
)]
#[post("/register")]
pub async fn register_user(
    user: web::Json<FormUser>,
    user_service: web::Data<UserService>,
    mailer_service: web::Data<Arc<Mutex<Mailer>>>,
) -> Result<()> {
    user.validate()?;

    if user_service
        .get_by_email_or_username(user.email.clone(), user.username.clone())
        .await?
        .is_some()
    {
        return Err(ErrorType::Conflict);
    }

    let user = user_service.create(user.into_inner()).await?;

    let message = user.get_email_jwt_url()?;

    match mailer_service.into_inner().lock().await.queue_mail(message).await {
        Ok(v) => Ok(v.into()),
        Err(either::Either::Left(e)) => Err(ErrorType::Database(DatabaseError::EmailSend(e))),
        Err(either::Either::Right(_)) => Err(ErrorType::Database(DatabaseError::Other(
            "Mailing service isn't started".to_string(),
        ))),
    }
}

/// Marks an account as verified/activated, given that the email token is valid
#[utoipa::path(
    context_path = "/api/v1/auth",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "Email verification token has expired"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
    ),
    params(
        EmailVerificationQuery
    )
)]
#[get("/verify")]
pub async fn verify_email(
    query: web::Query<EmailVerificationQuery>,
    user_service: web::Data<UserService>,
) -> Result<()> {
    let Ok(claims) = UserResource::verify_email_jwt(&query.email_key) else {
        return Err(ErrorType::Invalid)
    };

    let JwtType::Email(user_id) = claims.dat else {
        return Err(ErrorType::Invalid)
    };

    let Some(time) = chrono::NaiveDateTime::from_timestamp_opt(claims.exp, 0) else {
        return Err(ErrorType::ServerError)
    };

    if chrono::Utc::now() < chrono::DateTime::<chrono::Utc>::from_utc(time, Utc) {
        let Some(mut user_res) = user_service.get_by_id(user_id).await? else {
            return Err(ErrorType::ServerError)
        };

        user_res.activate().await?;

        Ok(().into())
    } else {
        Err(ErrorType::Expired)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login_user)
            .service(register_user)
            .service(verify_email),
    );
}
