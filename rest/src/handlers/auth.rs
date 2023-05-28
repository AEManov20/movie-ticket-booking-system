use super::*;
use chrono::Utc;

use crate::{
    model::{FormUser, JwtType, LoginUser, User},
    services::user::{UserResource, UserService},
};
use super::ErrorResponse;

#[derive(Serialize)]
#[serde(tag = "type")]
enum LoginErrorType {
    IncorrectEmailOrPassword,
    UserRegisteredExternally,
    UserNotVerified,
    ValidationErrors(ValidationErrors),
    FailedCreatingTokens,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum RegisterErrorType {
    ValidationErrors(ValidationErrors),
    UserAlreadyExists,
    FailedRegisteringUser,
    FailedCreatingEmailToken,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum VerifyErrorType {
    InvalidToken,
    TokenExpired,
    UserDoesntExist,
    TimeIsBroken,
}

#[derive(Deserialize)]
struct EmailVerificationQuery {
    email_key: String,
}

#[get("/login")]
async fn login_user(
    params: web::Query<LoginUser>,
    user_service: web::Data<UserService>,
) -> HttpResponse {
    if let Err(e) = params.validate() {
        return HttpResponse::BadRequest().json(&ErrorResponse {
            error: LoginErrorType::ValidationErrors(e),
        });
    }

    let Ok(Some(user_res)) = user_service.get_by_email_or_username(Some(params.email.clone()), None).await else {
        return HttpResponse::BadRequest().json(&ErrorResponse { error: LoginErrorType::IncorrectEmailOrPassword })
    };

    let user = User::from(user_res.clone());

    if !user.is_activated {
        return HttpResponse::BadRequest().json(&ErrorResponse {
            error: LoginErrorType::UserNotVerified,
        });
    }

    let Some(password_hash) = user.password_hash else {
        return HttpResponse::BadRequest().json(&ErrorResponse {
            error: LoginErrorType::UserRegisteredExternally
        });
    };

    if crate::password::verify(params.password.as_bytes(), &password_hash) {
        match user_res.create_jwt() {
            Ok(ref res) => HttpResponse::Ok().json(res),
            Err(e) => {
                error!("{:?}", e);

                HttpResponse::InternalServerError().json(&ErrorResponse {
                    error: LoginErrorType::FailedCreatingTokens,
                })
            }
        }
    } else {
        HttpResponse::BadRequest().json(&ErrorResponse {
            error: LoginErrorType::IncorrectEmailOrPassword,
        })
    }
}

#[post("/register")]
async fn register_user(
    user: web::Json<FormUser>,
    user_service: web::Data<UserService>,
) -> HttpResponse {
    let Ok(None) = user_service.get_by_email_or_username(Some(user.email.clone()), Some(user.username.clone())).await else {
        return HttpResponse::Conflict().json(ErrorResponse { error: RegisterErrorType::UserAlreadyExists })
    };

    match user.validate() {
        Ok(_) => {
            let result = user_service.create(user.into_inner()).await.or_else(|x| {
                error!("{}", x);
                Err(x)
            });
            let Ok(Some(user_res)) = result else {
                return HttpResponse::InternalServerError().json(&ErrorResponse {
                    error: RegisterErrorType::FailedRegisteringUser
                });
            };

            let result = user_res.create_email_jwt().or_else(|x| {
                error!("{}", x);
                Err(x)
            });
            let Ok(email_token) = result else {
                return HttpResponse::InternalServerError().json(&ErrorResponse {
                    error: RegisterErrorType::FailedCreatingEmailToken
                });
            };

            HttpResponse::Ok().json(email_token)
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: RegisterErrorType::ValidationErrors(e),
        }),
    }
}

#[get("/verify")]
async fn verify_email(
    query: web::Query<EmailVerificationQuery>,
    user_service: web::Data<UserService>,
) -> HttpResponse {
    let Some(claims) = UserResource::verify_email_jwt(&query.email_key) else {
        return HttpResponse::Forbidden().json(ErrorResponse { error: VerifyErrorType::InvalidToken });
    };

    let JwtType::Email(user_id) = claims.dat else {
        return HttpResponse::Forbidden().json(ErrorResponse { error: VerifyErrorType::InvalidToken });
    };

    let Some(time) = chrono::NaiveDateTime::from_timestamp_opt(claims.exp as i64, 0) else {
        return HttpResponse::InternalServerError().json(ErrorResponse { error: VerifyErrorType::TimeIsBroken });
    };

    if chrono::Utc::now() < chrono::DateTime::<chrono::Utc>::from_utc(time, Utc) {
        let result = user_service.get_by_id(user_id).await;

        match result {
            Ok(v) => {
                let Some(user_res) = v else {
                    return HttpResponse::InternalServerError().json(ErrorResponse { error: VerifyErrorType::UserDoesntExist })
                };

                match user_res.activate().await {
                    Ok(_) => HttpResponse::Ok().into(),
                    Err(e) => {
                        error!("{}", e);
                        HttpResponse::InternalServerError().into()
                    }
                }
            },
            Err(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().into()
            }
        }
    } else {
        HttpResponse::BadRequest().json(ErrorResponse { error: VerifyErrorType::TokenExpired })
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
