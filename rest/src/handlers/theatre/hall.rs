use crate::model::{FormHall, Hall};

use super::*;

/// Gets all halls for a given theatre ID
#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/hall",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = OK, description = "The selected theatre was found and its halls were returned", body = Vec<Hall>)
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre")
    )
)]
#[get("/all")]
pub async fn get_halls(
    path: web::Path<uuid::Uuid>,
    theatre_service: web::Data<TheatreService>,
) -> Result<Vec<Hall>> {
    let theatre_id = path.into_inner();
    let Some(theatre) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    Ok(theatre
        .get_halls()
        .await?
        .iter()
        .map(|x| Hall::from(x.to_owned()))
        .collect::<Vec<_>>()
        .into())
}

/// Creates a new hall
#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/hall",
    request_body = FormHall,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "The selected theatre was found and the hall was created and returned", body = Hall)
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre")
    ),
    security(
        ("api_key" = [])
    )
)]
#[post("/new")]
pub async fn create_hall(
    path: web::Path<uuid::Uuid>,
    new_hall: web::Json<FormHall>,
    theatre_service: web::Data<TheatreService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Hall> {
    new_hall.validate()?;

    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let theatre_id = path.into_inner();
    let Some(theatre) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles_or!(
            [Role::TheatreOwner],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    let hall: Hall = theatre.create_hall(new_hall.into_inner()).await?.into();
    Ok(hall.into())
}

/// Deletes a hall
#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/hall",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = OK, description = "The selected theatre was found and the hall was created and returned", body = Hall)
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre"),
        ("hid", description = "Unique storage ID for Hall")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/{hid}")]
pub async fn delete_hall(
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    theatre_service: web::Data<TheatreService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<()> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let (theatre_id, hall_id) = path.into_inner();
    let Some(theatre) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles_or!(
            [Role::TheatreOwner],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(theatre.delete_hall(hall_id).await?.into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hall")
            .service(get_halls)
            .service(create_hall)
            .service(delete_hall),
    );
}
