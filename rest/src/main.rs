mod handlers;
mod model;
mod password;
mod schema;
mod services;
mod util;
mod vars;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use services::{movie::MovieService, theatre::TheatreService, user::UserService};
use util::get_connection_pool;

async fn root_response() -> impl Responder {
    HttpResponse::Ok().body("*wind noises*")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let pool = get_connection_pool();

    let movie_service = MovieService::new(pool.clone());
    let theatre_service = TheatreService::new(pool.clone());
    let user_service = UserService::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(movie_service.clone()))
            .app_data(web::Data::new(theatre_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .route("/", web::get().to(root_response))
            .service(
                web::scope("/api/v1")
                    .configure(handlers::config)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
