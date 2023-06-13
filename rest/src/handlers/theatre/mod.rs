use crate::{
    check_roles,
    model::{FormTheatre, Role, Theatre},
    services::{bridge_role::BridgeRoleService, role::RoleService, theatre::TheatreService},
};

use super::*;

mod role;
mod screening;
mod ticket;
mod ticket_type;
mod hall;

#[post("/new")]
async fn new_theatre(
    theatre: web::Json<FormTheatre>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Theatre> {
    let (user_res, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        return Err(ErrorType::InsufficientPermission);
    }

    Ok(Theatre::from(theatre_service.create(theatre.into_inner()).await?).into())
}

#[get("/{id}")]
async fn get_theatre(
    path: web::Path<(uuid::Uuid,)>,
    theatre_service: web::Data<TheatreService>,
) -> Result<Theatre> {
    match theatre_service.get_by_id(path.0).await?.map(Theatre::from) {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound),
    }
}

#[put("/{id}")]
async fn update_theatre(
    path: web::Path<(uuid::Uuid,)>,
    form: web::Json<FormTheatre>,
    theatre_service: web::Data<TheatreService>,
    role_service: web::Data<RoleService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Theatre> {
    let theatre_id = path.0;
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        check_roles!(
            [Role::TheatreOwner],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(Theatre::from(
        theatre_service
            .update(theatre_id, form.into_inner())
            .await?,
    )
    .into())
}

#[delete("/{id}")]
async fn delete_theatre(
    path: web::Path<(uuid::Uuid,)>,
    theatre_service: web::Data<TheatreService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<()> {
    let theatre_id = path.0;
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if !user.is_super_user {
        return Err(ErrorType::InsufficientPermission);
    }

    Ok(theatre_service.delete(theatre_id).await?.into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/theatre")
            .service(
                web::scope("/{id}")
                    .configure(screening::config)
                    .configure(role::config)
                    .configure(ticket_type::config)
                    .configure(ticket::config)
                    .configure(hall::config),
            )
            .service(new_theatre)
            .service(get_theatre)
            .service(update_theatre)
            .service(delete_theatre),
    );
}
