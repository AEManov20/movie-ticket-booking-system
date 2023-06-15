use crate::{
    check_roles, doc,
    model::{FormTheatre, Role, Theatre},
    services::{bridge_role::*, role::*, theatre::*},
};

use super::*;

pub mod hall;
pub mod role;
pub mod screening;
pub mod ticket;
pub mod ticket_type;

#[utoipa::path(
    context_path = "/api/v1/theatre",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)", body = DocError, example = json!(doc!(ErrorType::Database(DatabaseError::Other("".to_string()))))),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet", body = DocError, example = json!(doc!(ErrorType::NoAuth))),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions", body = DocError, example = json!(doc!(ErrorType::InsufficientPermission))),
        (status = OK, description = "A new theatre was successfully created", body = Theatre)
    )
)]
#[post("/new")]
pub async fn new_theatre(
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

#[utoipa::path(context_path = "/api/v1/theatre")]
#[get("/{id}")]
pub async fn get_theatre(
    path: web::Path<(uuid::Uuid,)>,
    theatre_service: web::Data<TheatreService>,
) -> Result<Theatre> {
    match theatre_service.get_by_id(path.0).await?.map(Theatre::from) {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound),
    }
}

#[utoipa::path(context_path = "/api/v1/theatre")]
#[put("/{id}")]
pub async fn update_theatre(
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

#[utoipa::path(context_path = "/api/v1/theatre")]
#[delete("/{id}")]
pub async fn delete_theatre(
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
            .service(new_theatre)
            .service(get_theatre)
            .service(update_theatre)
            .service(delete_theatre)
            .service(
                web::scope("/{id}")
                    .configure(screening::config)
                    .configure(role::config)
                    .configure(ticket_type::config)
                    .configure(ticket::config)
                    .configure(hall::config),
            ),
    );
}
