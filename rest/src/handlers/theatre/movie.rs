use super::*;

#[get("/available")]
async fn get_theatre_movies() -> HttpResponse {

    
    HttpResponse::Ok().into()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/movie")
            .service(get_theatre_movies)
    );
}
