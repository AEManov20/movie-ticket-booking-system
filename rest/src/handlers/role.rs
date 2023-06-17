use std::collections::HashMap;

use rayon::prelude::*;

use utoipa::ToSchema;

use crate::{
    model::{Role, UserTheatreRole},
    services::{bridge_role::BridgeRoleService, role::RoleService}, check_roles_or,
};

use super::*;

#[derive(Deserialize, ToSchema)]
pub struct BridgeRoleQuery {
    pub role_id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub theatre_id: uuid::Uuid,
}

/// Gets available roles in the form of a dictionary
#[utoipa::path(context_path = "/api/v1/role")]
#[get("/available")]
pub async fn get_all_roles(
    role_service: web::Data<RoleService>,
) -> Result<HashMap<String, uuid::Uuid>> {
    Ok(role_service
        .get_all_roles()
        .await?
        .par_iter()
        .map(|el| (el.name.clone(), el.id))
        .collect::<HashMap<String, uuid::Uuid>>()
        .into())
}

/// Queries assigned user roles and the theatre they're linked to
#[utoipa::path(context_path = "/api/v1/role")]
#[get("/query_bridge")]
pub async fn query_bridge_roles(
    query: web::Query<BridgeRoleQuery>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Vec<UserTheatreRole>> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        check_roles_or!(
            [Role::TheatreOwner, Role::UserManager],
            user.id,
            query.theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(bridge_role_service
        .get_roles(Some(query.role_id), query.user_id, Some(query.theatre_id))
        .await?
        .into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/role")
            .service(get_all_roles)
            .service(query_bridge_roles),
    );
}
