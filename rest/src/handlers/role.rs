use std::collections::HashMap;

use crate::{
    model::UserTheatreRole,
    services::{bridge_role::BridgeRoleService, role::RoleService},
};

use super::*;

#[derive(Deserialize)]
struct BridgeRoleQuery {
    role_id: Option<uuid::Uuid>,
    user_id: Option<uuid::Uuid>,
    theatre_id: Option<uuid::Uuid>,
}

#[get("/available")]
async fn get_all_roles(
    role_service: web::Data<RoleService>,
) -> Result<HashMap<String, uuid::Uuid>> {
    Ok(role_service
        .get_all_roles()
        .await?
        .iter()
        .map(|el| (el.name.clone(), el.id))
        .collect::<HashMap<String, uuid::Uuid>>()
        .into())
}

#[get("/query")]
async fn query_bridge_roles(
    query: web::Query<BridgeRoleQuery>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims
) -> Result<Vec<UserTheatreRole>> {
    let (user_res, user) = user_res_from_jwt(&claims, &user_service).await?;

    Ok(bridge_role_service.get_roles(query.role_id, query.user_id, query.theatre_id).await?.into())
}

#[post("/new")]
async fn register_bridge_role(role_service: web::Data<RoleService>) -> Result<UserTheatreRole> {
    todo!();
}

#[delete("/{id}")]
async fn unregister_bridge_role(role_service: web::Data<RoleService>) -> Result<UserTheatreRole> {
    todo!();
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/role")
            .service(get_all_roles)
            .service(query_bridge_roles)
            .service(register_bridge_role)
            .service(unregister_bridge_role),
    );
}
