use crate::model::{CreateTicketType, FormTicketType, TicketType};

use super::*;

/// Gets different ticket types/pricings
#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/ticket_type",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = OK, description = "The selected theatre was found and the TicketTypes were returned", body = Vec<TicketType>)
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID for Theatre"),
    ),
)]
#[get("/all")]
pub async fn get_all_ticket_types(
    path: web::Path<uuid::Uuid>,
    theatre_service: web::Data<TheatreService>,
) -> Result<Vec<TicketType>> {
    let Some(theatre_res) = theatre_service.get_by_id(path.into_inner()).await? else {
        return Err(ErrorType::NotFound)
    };

    Ok(theatre_res.get_ticket_types().await?.into())
}

/// Creates a new ticket pricing/type
#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/ticket_type",
    request_body = FormTicketType,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner || TicketManager)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "The selected theatre was found and new TicketType was created", body = TicketType)
    ),
    security(
        ("api_key" = [])
    )
)]
#[post("/new")]
pub async fn create_ticket_type(
    path: web::Path<uuid::Uuid>,
    new_ticket_type: web::Json<FormTicketType>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims,
) -> Result<TicketType> {
    let theatre_id = path.into_inner();
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles_or!(
            [Role::TheatreOwner, Role::TicketManager],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(theatre_res
        .create_ticket_type(CreateTicketType::from_form(
            new_ticket_type.into_inner(),
            theatre_id,
        ))
        .await?
        .into())
}

/// Deletes a ticket pricing/type
#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/ticket_type",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner || TicketManager)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = OK, description = "The selected theatre was found and the TicketType was deleted")
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID for Theatre"),
        ("ttid", description = "Unique storage ID for TicketType")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/{ttid}")]
pub async fn delete_ticket_type(
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims,
) -> Result<()> {
    let (theatre_id, ticket_type_id) = path.into_inner();
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles_or!(
            [Role::TheatreOwner, Role::TicketManager],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(theatre_res.delete_ticket_type(ticket_type_id).await?.into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ticket_type")
            .service(get_all_ticket_types)
            .service(create_ticket_type)
            .service(delete_ticket_type),
    );
}
