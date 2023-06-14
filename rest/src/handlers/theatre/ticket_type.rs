use crate::model::{TicketType, FormTicketType};

use super::*;

#[get("/all")]
pub async fn get_all_ticket_types(
    path: web::Path<uuid::Uuid>,
    theatre_service: web::Data<TheatreService>
) -> Result<Vec<TicketType>> {
    let Some(theatre_res) = theatre_service.get_by_id(path.into_inner()).await? else {
        return Err(ErrorType::NotFound)
    };

    Ok(theatre_res.get_ticket_types().await?.into())
}

#[post("/new")]
pub async fn create_ticket_type(
    path: web::Path<uuid::Uuid>,
    new_ticket_type: web::Json<FormTicketType>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims
) -> Result<TicketType> {
    let theatre_id = path.into_inner();
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles!(
            [Role::TheatreOwner, Role::TicketManager],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(theatre_res.create_ticket_type(new_ticket_type.into_inner()).await?.into())
}

#[delete("/{ttid}")]
pub async fn delete_ticket_type(
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    claims: JwtClaims
) -> Result<()> {
    let (theatre_id, ticket_type_id) = path.into_inner();
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles!(
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
    cfg.service(web::scope("/ticket_type"));
}
