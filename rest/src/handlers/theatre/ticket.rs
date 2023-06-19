use utoipa::IntoParams;

use crate::model::Ticket;

use super::*;

// TODO!

#[derive(Validate, Deserialize, ToSchema, IntoParams)]
pub struct TicketQuery {
    pub issuer_id: Option<uuid::Uuid>,
    pub ticket_type: Option<uuid::Uuid>,
    pub theatre_movie_id: Option<uuid::Uuid>,
    pub user_id: Option<uuid::Uuid>,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
    pub offset: i64,
}

#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/ticket",
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre"),
        TicketQuery
    ),

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
    // query.validate()?;

    // let theatre_id = path.into_inner();
    // let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    // if !user.is_super_user {
    //     check_roles_or!(
    //         [Role::TheatreOwner, Role::TicketManager],
    //         user.id,
    //         theatre_id,
    //         bridge_role_service,
    //         role_service
    //     );
    // }

    
    // Ok(vec![])

    todo!();
}

#[utoipa::path]
#[post("/new")]
pub async fn create_ticket() -> Result<Ticket> {
    todo!();
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/ticket"));
}
