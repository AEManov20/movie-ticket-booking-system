mod auth_handler;
mod model;
mod password;
mod schema;
mod vars;
mod util;
mod services;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use services::{user::UserService, movie::MovieService, theatre::TheatreService};
use util::{get_connection_pool, hash_mock_passwords};

async fn root_response() -> impl Responder {
    HttpResponse::Ok().body("*wind noises*")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = get_connection_pool();

    let user_service = UserService::new(pool.clone());
    // let movie_service = MovieService::new(pool.clone());
    // let theatre_service = TheatreService::new(pool.clone());

    // HttpServer::new(move || {
    //     App::new()
    //         // .app_data(web::Data::new(pool.clone()))
    //         .route("/", web::get().to(root_response))
    //         .service(web::scope("/api"))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await?;

    Ok(())
}
