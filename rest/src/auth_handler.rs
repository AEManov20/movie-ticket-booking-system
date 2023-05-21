use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/login")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().into()
}

#[post("/register")]
async fn register() -> HttpResponse {
    HttpResponse::Ok().into()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(login).service(register));
}
