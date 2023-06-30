use utoipa::IntoParams;

use crate::{
    model::{FormTicket, Ticket, TicketQuery},
    services::user::TicketResource,
};

use super::*;

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct NewTicketQuery {
    pub owner_id: Option<uuid::Uuid>,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ValidateTicketQuery {
    pub ticket_jwt: String,
}

async fn validate_and_get(
    theatre_id: web::Path<uuid::Uuid>,
    query: web::Query<ValidateTicketQuery>,
    user_service: web::Data<UserService>,
    theatre_service: web::Data<TheatreService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims,
) -> std::result::Result<(TicketResource, Ticket), ErrorType> {
    let theatre_id = theatre_id.into_inner();
    let ticket_id = TicketResource::verify_jwt(&query.ticket_jwt)?;
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles_or!(
            [Role::TheatreOwner, Role::TicketChecker],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    let Some(ticket_res) = theatre_res.get_ticket_by_id(ticket_id).await? else {
        return Err(ErrorType::ServerError)
    };
    let ticket = Ticket::from(ticket_res.clone());

    if chrono::Utc::now().naive_utc() > ticket.expires_at {
        Err(ErrorType::Expired)
    } else {
        Ok((ticket_res, ticket))
    }
}

#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/ticket",
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre"),
        TicketQuery
    ),
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner || TicketManager)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "The selected theatre was found and the query returned", body = Vec<Ticket>)
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/query")]
pub async fn query_tickets(
    path: web::Path<uuid::Uuid>,
    query: web::Query<TicketQuery>,
    user_service: web::Data<UserService>,
    theatre_service: web::Data<TheatreService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims,
) -> Result<Vec<Ticket>> {
    query.validate()?;

    let theatre_id = path.into_inner();
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre) = theatre_service.get_by_id(theatre_id).await? else {
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

    Ok(theatre.query_tickets(query.into_inner()).await?.into())
}

#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/ticket",
    request_body = FormTicket,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner || TicketManager)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "The selected theatre was found and the ticket was created", body = Vec<Ticket>)
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre"),
        NewTicketQuery
    ),
    security(
        ("api_key" = [])
    )
)]
#[post("/new")]
pub async fn create_ticket(
    path: web::Path<uuid::Uuid>,
    query: web::Query<NewTicketQuery>,
    new_ticket: web::Json<FormTicket>,
    user_service: web::Data<UserService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims,
) -> Result<Ticket> {
    let theatre_id = path.into_inner();
    let (issuer_user_res, issuer_user) = user_res_from_jwt(&claims, &user_service).await?;
    
    if let Some(owner_id) = query.owner_id {
        if !issuer_user.is_super_user && owner_id != issuer_user.id {
            check_roles_or!(
                [Role::TheatreOwner, Role::TicketManager],
                issuer_user.id,
                theatre_id,
                bridge_role_service,
                role_service
            );

            let Some(receiver_user_res) = user_service.get_by_id(owner_id).await? else {
                return Err(ErrorType::NotFound)
            };

            return Ok(Ticket::from(
                receiver_user_res
                    .create_ticket(new_ticket.into_inner(), issuer_user.id)
                    .await?,
            )
            .into());
        }
    }

    // TODO: implement check whether the ticket type belongs to the theatre

    let owned_ticket_count = issuer_user_res
        .get_tickets_count(Some(new_ticket.theatre_screening_id))
        .await?;
    if owned_ticket_count > 3 {
        return Err(ErrorType::InsufficientPermission);
    }

    Ok(Ticket::from(
        issuer_user_res
            .create_ticket(new_ticket.into_inner(), issuer_user.id)
            .await?,
    )
    .into())
}

#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/ticket",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet or ticket has expired"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner || TicketManager)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "The selected theatre was found and the ticket was validated", body = Vec<Ticket>)
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre"),
        NewTicketQuery
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/validate")]
pub async fn validate(
    path: web::Path<uuid::Uuid>,
    query: web::Query<ValidateTicketQuery>,
    user_service: web::Data<UserService>,
    theatre_service: web::Data<TheatreService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims,
) -> Result<Ticket> {
    Ok(validate_and_get(
        path,
        query,
        user_service,
        theatre_service,
        role_service,
        bridge_role_service,
        claims,
    )
    .await?
    .1
    .into())
}

#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/ticket",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet or ticket has expired"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner || TicketManager)"),
        (status = NOT_FOUND, description = "The selected theatre was not found"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "The selected theatre was found and the ticket was validated and marked", body = Vec<Ticket>)
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre"),
        ValidateTicketQuery
    ),
    security(
        ("api_key" = [])
    )
)]
#[put("/mark/{state}")]
pub async fn validate_and_mark(
    path: web::Path<(uuid::Uuid, bool)>,
    query: web::Query<ValidateTicketQuery>,
    user_service: web::Data<UserService>,
    theatre_service: web::Data<TheatreService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims,
) -> Result<()> {
    let (theatre_id, state) = path.into_inner();

    let (mut ticket_res, _) = validate_and_get(
        web::Path::from(theatre_id),
        query,
        user_service,
        theatre_service,
        role_service,
        bridge_role_service,
        claims,
    )
    .await?;

    match state {
        true => ticket_res.mark_as_used().await?,
        false => ticket_res.mark_as_unused().await?,
    };

    Ok(().into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ticket")
            .service(query_tickets)
            .service(create_ticket)
            .service(validate)
            .service(validate_and_mark),
    );
}
