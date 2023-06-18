use utoipa::IntoParams;

use crate::model::{ExtendedMovieReview, ExtendedUserReview, MovieReview, PartialUser, Ticket};

use super::*;

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct NewPasswordForm {
    pub old_password: String,
    pub new_password: String,
    pub new_password_repeat: String,
}

#[utoipa::path(
    context_path = "/api/v1/user",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = OK, description = "User found and returned", body = User)
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/@me")]
pub async fn get_self_user(
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<User> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    Ok(user.into())
}

#[utoipa::path(
    context_path = "/api/v1/user",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = OK, description = "Tickets are returned", body = Vec<Ticket>)
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/@me/tickets")]
pub async fn get_self_tickets(
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Vec<Ticket>> {
    let (user_res, _) = user_res_from_jwt(&claims, &user_service).await?;

    Ok(user_res
        .get_tickets()
        .await?
        .iter()
        .cloned()
        .map(Ticket::from)
        .collect::<Vec<_>>()
        .into())
}

#[utoipa::path(
    context_path = "/api/v1/user",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = OK, description = "Reviews are returned", body = Vec<ExtendedMovieReview>)
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/@me/reviews")]
pub async fn get_self_reviews(
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Vec<ExtendedMovieReview>> {
    let (user_res, _) = user_res_from_jwt(&claims, &user_service).await?;

    Ok(user_res.get_reviews().await?.into())
}

#[utoipa::path(
    context_path = "/api/v1/user",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "User password updated successfully")
    ),
    security(
        ("api_key" = [])
    )
)]
#[put("/@me/password")]
pub async fn update_self_password(
    form: web::Json<NewPasswordForm>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<()> {
    let (mut user_res, user) = user_res_from_jwt(&claims, &user_service).await?;

    if let Some(hash) = user.password_hash {
        if !crate::password::verify(form.old_password.as_bytes(), &hash) {
            return Err(ErrorType::Invalid);
        }
    }

    if form.new_password != form.new_password_repeat {
        return Err(ErrorType::Invalid);
    }

    Ok(user_res
        .update_password(form.new_password.clone())
        .await?
        .into())
}

#[utoipa::path(
    context_path = "/api/v1/user",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = NOT_FOUND, description = "User wasn't found"),
        (status = OK, description = "User found and returned", body = PartialUser)
    )
)]
#[get("/{id}")]
pub async fn get_partial_user(
    path: web::Path<uuid::Uuid>,
    user_service: web::Data<UserService>,
) -> Result<PartialUser> {
    match user_service
        .get_by_id(path.into_inner())
        .await?
        .map(User::from)
        .map(PartialUser::from)
    {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound),
    }
}

#[utoipa::path(
    context_path = "/api/v1/user",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = NOT_FOUND, description = "User wasn't found"),
        (status = OK, description = "User found and returned", body = Vec<ExtendedMovieReview>)
    )
)]
#[get("/{id}/reviews")]
pub async fn get_user_reviews(
    path: web::Path<uuid::Uuid>,
    user_service: web::Data<UserService>,
) -> Result<Vec<ExtendedMovieReview>> {
    let Some(user_res) = user_service.get_by_id(path.into_inner()).await? else {
        return Err(ErrorType::NotFound)
    };

    Ok(user_res.get_reviews().await?.into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(get_self_user)
            .service(get_self_tickets)
            .service(update_self_password)
            .service(get_partial_user)
            .service(get_user_reviews),
    );
}
