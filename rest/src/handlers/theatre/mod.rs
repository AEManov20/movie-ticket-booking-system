use crate::model::FormTheatre;

use super::*;

mod movie;
mod permission;
mod ticket_type;
mod ticket;

#[post("/new")]
async fn new_theatre(theatre: web::Json<FormTheatre>) -> HttpResponse {
    todo!();
}

#[get("/{id}")]
async fn get_theatre_by_id(path: web::Path<(i32,)>) -> HttpResponse {
    todo!();
}

#[put("/{id}")]
async fn update_theatre_by_id(path: web::Path<(i32,)>) -> HttpResponse {
    todo!();
}

#[delete("/{id}")]
async fn delete_theatre_by_id(path: web::Path<(i32,)>) -> HttpResponse {
    todo!();
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/theatre")
            .service(web::scope("/{id}")
                .configure(movie::config)
                .configure(permission::config)
                .configure(ticket_type::config)
                .configure(ticket::config))
            .service(new_theatre)
            .service(get_theatre_by_id)
            .service(update_theatre_by_id)
            .service(delete_theatre_by_id)
    );
}