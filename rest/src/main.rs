mod handlers;
mod model;
mod password;
mod schema;
mod services;
mod util;
mod vars;

mod doc;

use actix_web::{web, App, HttpResponse, HttpServer, error::ErrorImATeapot};
use services::{movie::MovieService, theatre::TheatreService, user::UserService, bridge_role::BridgeRoleService, role::RoleService, language::LanguageService};
use util::get_connection_pool;

async fn root_response() -> HttpResponse {
    ErrorImATeapot("*wind noises*").into()
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let pool = get_connection_pool();

    let movie_service = MovieService::new(pool.clone());
    let theatre_service = TheatreService::new(pool.clone());
    let user_service = UserService::new(pool.clone());
    let bridge_role_service = BridgeRoleService::new(pool.clone());
    let role_service = RoleService::new(pool.clone());
    let language_service = LanguageService::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(movie_service.clone()))
            .app_data(web::Data::new(theatre_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(bridge_role_service.clone()))
            .app_data(web::Data::new(role_service.clone()))
            .app_data(web::Data::new(language_service.clone()))
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
