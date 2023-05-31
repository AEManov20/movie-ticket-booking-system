use crate::{
    model::{Theatre, TheatrePermission, User},
    services::theatre::TheatreService,
};

use super::*;

#[get("/all")]
async fn get_all_permissions(
    path: web::Path<(uuid::Uuid,)>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Vec<TheatrePermission>> {
    let (user_res, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre) = theatre_service.get_by_id(path.0).await? else {
        return Err(ErrorType::NotFound)
    };

    if user.is_super_user {
        return Ok(theatre.get_permissions().await?.into());
    }

    let Some(permission) = user_res.get_theatre_permission(path.0).await? else {
        return Err(ErrorType::InsufficientPermission)
    };

    if permission.is_theatre_owner || permission.can_manage_users {
        Ok(theatre.get_permissions().await?.into())
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

#[get("/{user_id}")]
async fn get_user_permission(
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<TheatrePermission> {
    let (req_user_res, req_user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre) = theatre_service.get_by_id(path.0).await? else {
        return Err(ErrorType::NotFound)
    };
    let theatre = Theatre::from(theatre);

    let (target_user_res, target_user) = if claims.sub == req_user.id {
        (req_user_res.clone(), req_user.clone())
    } else {
        match user_service.get_by_id(path.1).await? {
            Some(v) => (v.clone(), User::from(v)),
            None => return Err(ErrorType::NotFound),
        }
    };

    if req_user.is_super_user {
        return match target_user_res.get_theatre_permission(theatre.id).await? {
            Some(v) => Ok(v.into()),
            None => Err(ErrorType::NotFound),
        };
    }

    let Some(req_user_perms) = req_user_res.get_theatre_permission(theatre.id).await? else {
        return Err(ErrorType::InsufficientPermission)
    };

    if req_user_perms.is_theatre_owner
        || req_user_perms.can_manage_users
        || target_user.id == req_user.id
    {
        match target_user_res.get_theatre_permission(path.0).await? {
            Some(v) => Ok(v.into()),
            None => Err(ErrorType::NotFound),
        }
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

#[put("/{user_id}")]
async fn update_user_permission(
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<TheatrePermission> {
    todo!();
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/permission"));
}
