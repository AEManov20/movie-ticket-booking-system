use crate::{model::User, services::theatre::TheatreService};

use super::*;

#[get("/all")]
async fn get_all_permissions(path: web::Path<(uuid::Uuid,)>, theatre_service: web::Data<TheatreService>, user_service: web::Data<UserService>, claims: JwtClaims) -> HttpResponse {
    // match theatre_service.get_by_id(path.0).await {
    //     Ok(Some(theatre)) => {
    //         match user_res_from_jwt(&claims, &user_service).await {
    //             Ok(user) => {
    //                 match user.get_theatre_permission(path.0).await {
    //                     Ok(Some(user_perms)) => {
    //                         if user_perms.can_manage_users {
    //                             match theatre_service.
    //                         } else {
    //                             HttpResponse::Forbidden().into()
    //                         }
    //                     },
    //                     Ok(None) => HttpResponse::NotFound().into()
    //                 }
    //             }
    //             Err(e) => HttpResponse::Forbidden().json(e)
    //         }
    //     },
    //     Ok(None) => HttpResponse::NotFound().into(),
    //     Err(e) => {
    //         error!("{:?}", e);
    //         HttpResponse::InternalServerError().into()
    //     }
    // }

    todo!();
}

// #[get("/{user_id}")]
// async fn get_user_permissions(path: web::Path<(uuid::Uuid, uuid::Uuid)>, )

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/permission")
    );
}
