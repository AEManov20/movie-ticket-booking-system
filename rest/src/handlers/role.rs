use std::collections::HashMap;

use crate::{
    model::{Role, UserTheatreRole},
    services::{bridge_role::BridgeRoleService, role::RoleService}, check_roles,
};

use super::*;

#[derive(Deserialize)]
struct BridgeRoleQuery {
    role_id: uuid::Uuid,
    user_id: Option<uuid::Uuid>,
    theatre_id: uuid::Uuid,
}

#[derive(Deserialize)]
struct BridgeRoleForm {
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
async fn query_bridge_roles(
    query: web::Query<BridgeRoleQuery>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Vec<UserTheatreRole>> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        check_roles!(
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

#[post("/new")]
async fn register_bridge_role(
    form: web::Json<BridgeRoleForm>,
    bridge_role_service: web::Data<BridgeRoleService>,
    role_service: web::Data<RoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Option<UserTheatreRole>> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        check_roles!(
            [Role::TheatreOwner, Role::UserManager],
            user.id,
            form.theatre_id,
            bridge_role_service,
            role_service
        );
    }

    let new_role = UserTheatreRole {
        user_id: form.user_id,
        role_id: form.role_id,
        theatre_id: form.theatre_id,
    };

    if bridge_role_service.role_exists(new_role.clone()).await? {
        return Err(ErrorType::Conflict);
    }

    Ok(bridge_role_service.register_role(new_role).await?.into())
}

#[delete("/{id}")]
async fn unregister_bridge_role(
    form: web::Json<BridgeRoleForm>,
    bridge_role_service: web::Data<BridgeRoleService>,
    role_service: web::Data<RoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<()> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        check_roles!(
            [Role::TheatreOwner, Role::UserManager],
            user.id,
            form.theatre_id,
            bridge_role_service,
            role_service
        );
    }

    let del = UserTheatreRole {
        user_id: form.user_id,
        role_id: form.role_id,
        theatre_id: form.theatre_id,
    };

    if !bridge_role_service.role_exists(del.clone()).await? {
        return Err(ErrorType::NotFound);
    }

    Ok(bridge_role_service
        .unregister_roles(Some(del.user_id), Some(del.theatre_id), Some(del.role_id))
        .await?
        .into())
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
