use std::collections::HashMap;

use crate::{model::UserTheatreRole, services::role::RoleService};

use super::*;

#[derive(Deserialize)]
struct BridgeRoleQuery {
    role_id: uuid::Uuid,
    user_id: uuid::Uuid,
    theatre_id: uuid::Uuid,
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
async fn query_bridge_roles(role_service: web::Data<RoleService>) -> Result<Vec<UserTheatreRole>> {
    todo!();
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
        web::scope("/movie")
            .service(get_all_roles)
            .service(query_bridge_roles)
            .service(register_bridge_role)
            .service(unregister_bridge_role),
    );
}
