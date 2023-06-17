use std::collections::HashMap;

use rayon::prelude::*;
use utoipa::ToSchema;

use crate::{
    check_roles_or,
    model::{Role, UserTheatreRole},
    services::{bridge_role::BridgeRoleService, role::RoleService},
};

use super::*;

#[derive(Deserialize, Serialize, ToSchema)]
pub enum RoleUpdateAction {
    Create,
    Delete,
}

#[derive(Deserialize, ToSchema)]
pub struct UserRoleForm {
    #[schema(example = json!(Action::Create))]
    pub action: RoleUpdateAction,
    pub user_id: uuid::Uuid,
    pub role_id: uuid::Uuid,
}

/// Fetches all assigned user roles in the scope of the selected theatre ID
#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/role",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)", body = DocError, example = json!(doc!(ErrorType::Database(DatabaseError::Other("".to_string()))))),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet", body = DocError, example = json!(doc!(ErrorType::NoAuth))),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner || UserManager)", body = DocError, example = json!(doc!(ErrorType::InsufficientPermission))),
        (status = NOT_FOUND, description = "The selected theatre was not found", body = DocError, example = json!(doc!(ErrorType::Database(DatabaseError::Other("".to_string()))))),
        (status = OK, description = "The selected theatre was found and the roles were returned (user_id, role_id)", body = HashMap<uuid::Uuid, uuid::Uuid>)
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre")
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/all")]
pub async fn get_all_roles(
    path: web::Path<(uuid::Uuid,)>,
    bridge_role_service: web::Data<BridgeRoleService>,
    role_service: web::Data<RoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<HashMap<uuid::Uuid, uuid::Uuid>> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let theatre_id = path.0;

    if !user.is_super_user {
        check_roles_or!(
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

/// Creates/Deletes assigned user roles in the scope of the selected theatre ID
#[utoipa::path(
    context_path = "/api/v1/theatre/{id}/role",
    request_body = Vec<UserRoleForm>,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)", body = DocError, example = json!(doc!(ErrorType::Database(DatabaseError::Other("".to_string()))))),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet", body = DocError, example = json!(doc!(ErrorType::NoAuth))),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions (in this case TheatreOwner || UserManager) or has tried to update its own role", body = DocError, example = json!(doc!(ErrorType::InsufficientPermission))),
        (status = NOT_FOUND, description = "The selected theatre was not found", body = DocError, example = json!(doc!(ErrorType::Database(DatabaseError::Other("".to_string()))))),
        (status = BAD_REQUEST, description = "Invalid data supplied", body = DocError),
        (status = OK, description = "The selected theatre was found and the roles were updated")
    ),
    params(
        ("id" = uuid::Uuid, description = "Unique storage ID of Theatre")
    ),
    security(
        ("api_key" = [])
    )
)]
#[put("/update")]
pub async fn update_roles_batch(
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
        check_roles_or!(
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
            RoleUpdateAction::Create => ins_roles.push(UserTheatreRole {
                role_id: x.role_id,
                user_id: x.user_id,
                theatre_id,
            }),
            RoleUpdateAction::Delete => del_roles.push(UserTheatreRole {
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
