use std::collections::HashMap;

use rayon::prelude::*;

use crate::{
    check_roles,
    model::{Role, UserTheatreRole},
    services::{bridge_role::BridgeRoleService, role::RoleService},
};

use super::*;

#[derive(Deserialize)]
enum Action {
    Create,
    Delete,
}

#[derive(Deserialize)]
struct UserRoleForm {
    action: Action,
    user_id: uuid::Uuid,
    role_id: uuid::Uuid,
}

/// returns a hashmap with (user_id, role_id)
#[get("/all")]
async fn get_all_roles(
    path: web::Path<(uuid::Uuid,)>,
    bridge_role_service: web::Data<BridgeRoleService>,
    role_service: web::Data<RoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<HashMap<uuid::Uuid, uuid::Uuid>> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let theatre_id = path.0;

    if !user.is_super_user {
        check_roles!(
            [Role::TheatreOwner, Role::UserManager],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(bridge_role_service
        .get_roles(None, Some(theatre_id), None)
        .await?
        .par_iter()
        .map(|el| (el.user_id, el.role_id))
        .collect::<HashMap<_, _>>()
        .into())
}

#[put("/update")]
async fn update_roles_batch(
    path: web::Path<(uuid::Uuid,)>,
    batch: web::Json<Vec<UserRoleForm>>,
    bridge_role_service: web::Data<BridgeRoleService>,
    role_service: web::Data<RoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<()> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let theatre_id = path.0;

    if !user.is_super_user {
        check_roles!(
            [Role::TheatreOwner, Role::UserManager],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );

        for role in batch.iter() {
            if role.user_id == user.id {
                return Err(ErrorType::InsufficientPermission);
            }
        }
    }

    let mut ins_roles = vec![];
    let mut del_roles = vec![];

    for x in batch.iter() {
        match &x.action {
            Action::Create => ins_roles.push(UserTheatreRole {
                role_id: x.role_id,
                user_id: x.user_id,
                theatre_id,
            }),
            Action::Delete => del_roles.push(UserTheatreRole {
                role_id: x.role_id,
                user_id: x.user_id,
                theatre_id,
            }),
        };
    }

    bridge_role_service.register_roles(ins_roles).await?;
    bridge_role_service
        .unregister_roles_batch(del_roles)
        .await?;

    Ok(().into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/role")
            .service(get_all_roles)
            .service(update_roles_batch),
    );
}
