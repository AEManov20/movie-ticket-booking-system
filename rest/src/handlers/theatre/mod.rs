use crate::{
    check_roles_or, doc,
    model::{FormTheatre, Role, Theatre, Point},
    services::{bridge_role::*, role::*, theatre::*},
};

use super::*;

pub mod hall;
pub mod role;
pub mod screening;
pub mod ticket;
pub mod ticket_type;

/// Creates a new theatre (superuser only)
#[utoipa::path(
    context_path = "/api/v1/theatre",
    request_body = FormTheatre,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions"),
        (status = OK, description = "A new theatre was successfully created", body = Theatre)
    ),
    security(
        ("api_key" = [])
    )
)]
#[post("/new")]
pub async fn new_theatre(
    theatre: web::Json<FormTheatre>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Theatre> {
    let (user_res, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        return Err(ErrorType::InsufficientPermission);
    }

    Ok(Theatre::from(theatre_service.create(theatre.into_inner()).await?).into())
}

/// Gets data about a theatre
#[utoipa::path(
    context_path = "/api/v1/theatre",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = NOT_FOUND, description = "Occurs when given ID wasn't found in the database"),
        (status = OK, description = "Resource was found and returned", body = Theatre)
    )
)]
#[get("/{id}")]
pub async fn get_theatre(
    path: web::Path<(uuid::Uuid,)>,
    theatre_service: web::Data<TheatreService>,
) -> Result<Theatre> {
    match theatre_service.get_by_id(path.0).await?.map(Theatre::from) {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound),
    }
}

/// Updates a theatre (superuser only)
#[utoipa::path(
    context_path = "/api/v1/theatre",
    request_body = FormTheatre,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "The selected theatre was successfully updated", body = Theatre)
    ),
    security(
        ("api_key" = [])
    )
)]
#[put("/{id}")]
pub async fn update_theatre(
    path: web::Path<(uuid::Uuid,)>,
    form: web::Json<FormTheatre>,
    theatre_service: web::Data<TheatreService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Theatre> {
    form.validate()?;

    let theatre_id = path.0;
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        check_roles_or!(
            [Role::TheatreOwner],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(Theatre::from(
        theatre_service
            .update(theatre_id, form.into_inner())
            .await?,
    )
    .into())
}

/// Deletes a theatre (superuser only)
#[utoipa::path(
    context_path = "/api/v1/theatre",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = OK, description = "The selected theatre was successfully updated", body = Theatre)
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/{id}")]
pub async fn delete_theatre(
    path: web::Path<(uuid::Uuid,)>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<()> {
    let theatre_id = path.0;
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        return Err(ErrorType::InsufficientPermission);
    }

    Ok(theatre_service.delete(theatre_id).await?.into())
}

/// Gets theatres nearby a location
#[utoipa::path(
    context_path = "/api/v1/theatre",
    params(Point),
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = OK, description = "No errors occurred and the query returned", body = Vec<Theatre>)
    )
)]
#[get("/available")]
pub async fn get_nearby(
    theatre_service: web::Data<TheatreService>,
    location: web::Query<Point>
) -> Result<Vec<Theatre>> {
    Ok(theatre_service.get_nearby(location.into_inner()).await?.into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/theatre")
            .service(get_nearby)
            .service(new_theatre)
            .service(get_theatre)
            .service(update_theatre)
            .service(delete_theatre)
            .service(
                web::scope("/{id}")
                    .configure(screening::config)
                    .configure(role::config)
                    .configure(ticket_type::config)
                    .configure(ticket::config)
                    .configure(hall::config),
            ),
    );
}
