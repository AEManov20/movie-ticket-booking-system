use super::*;
use chrono::Utc;

use crate::{
    model::{FormUser, JwtType, LoginUser, User},
    services::user::{UserResource, UserService, LoginResponse},
};
use super::ErrorType;

#[derive(Deserialize)]
struct EmailVerificationQuery {
    email_key: String,
}

#[get("/login")]
async fn login_user(
    params: web::Query<LoginUser>,
    user_service: web::Data<UserService>,
) -> Result<LoginResponse> {
    params.validate()?;

    let Some(user_res) = user_service.get_by_email_or_username(Some(params.email.clone()), None).await? else {
        return Err(ErrorType::Invalid)
    };

    let user = User::from(user_res.clone());

    if !user.is_activated {
        return Err(ErrorType::EmailNotVerified)
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

#[post("/register")]
async fn register_user(
    user: web::Json<FormUser>,
    user_service: web::Data<UserService>,
) -> Result<String> {
    user.validate()?;
    
    let Ok(None) = user_service.get_by_email_or_username(Some(user.email.clone()), Some(user.username.clone())).await else {
        return Err(ErrorType::Conflict)
    };

    let Some(user) = user_service.create(user.into_inner()).await? else {
        return Err(ErrorType::ServerError)
    };

    Ok(user.create_email_jwt()?.into())
}

#[get("/verify")]
async fn verify_email(
    query: web::Query<EmailVerificationQuery>,
    user_service: web::Data<UserService>,
) -> Result<()> {
    let Ok(claims) = UserResource::verify_email_jwt(&query.email_key) else {
        return Err(ErrorType::Invalid)
    };

    let JwtType::Email(user_id) = claims.dat else {
        return Err(ErrorType::Invalid)
    };

    let Some(time) = chrono::NaiveDateTime::from_timestamp_opt(claims.exp as i64, 0) else {
        return Err(ErrorType::ServerError)
    };

    if chrono::Utc::now() < chrono::DateTime::<chrono::Utc>::from_utc(time, Utc) {
        let Some(user_res) = user_service.get_by_id(user_id).await? else {
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
