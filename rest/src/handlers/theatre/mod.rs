use utoipa::IntoParams;

use crate::{
    check_roles_or,
    model::{ExtendedTheatre, FormTheatre, Point, Role, Theatre},
    services::{bridge_role::*, role::*, theatre::*},
};

use super::*;

pub mod hall;
pub mod role;
pub mod screening;
pub mod ticket;
pub mod ticket_type;

#[derive(Deserialize, IntoParams)]
pub struct TheatreSearchQuery {
    pub name: String,
}

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
) -> HandlerResult<Theatre> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

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
        (status = OK, description = "Resource was found and returned", body = ExtendedTheatre)
    )
)]
#[get("/{id}")]
pub async fn get_theatre(
    path: web::Path<(uuid::Uuid,)>,
    theatre_service: web::Data<TheatreService>,
) -> HandlerResult<ExtendedTheatre> {
    match theatre_service.get_by_id_extra(path.0).await? {
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
) -> HandlerResult<Theatre> {
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
) -> HandlerResult<()> {
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
        (status = OK, description = "No errors occurred and the query returned", body = Vec<ExtendedTheatre>)
    )
)]
#[get("/available")]
pub async fn get_nearby(
    theatre_service: web::Data<TheatreService>,
    location: web::Query<Point>,
) -> HandlerResult<Vec<ExtendedTheatre>> {
    Ok(theatre_service
        .get_nearby_extended(location.into_inner())
        .await?
        .into())
}

#[utoipa::path(
    context_path = "/api/v1/theatre",
    params(TheatreSearchQuery),
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = OK, description = "No errors occurred and the query returned", body = Vec<ExtendedTheatre>)
    )
)]
#[get("/search")]
pub async fn search_by_name(
    theatre_service: web::Data<TheatreService>,
    query: web::Query<TheatreSearchQuery>,
) -> HandlerResult<Vec<ExtendedTheatre>> {
    Ok(theatre_service
        .get_by_name_extra(query.name.clone())
        .await?
        .into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/theatre")
            .service(search_by_name)
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
