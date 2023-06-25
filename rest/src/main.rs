mod handlers;
mod model;
mod password;
mod schema;
mod services;
mod util;
mod vars;

mod doc;

use actix_web::{error::ErrorImATeapot, web, App, HttpResponse, HttpServer};
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use services::{
    bridge_role::BridgeRoleService, language::LanguageService, movie::MovieService,
    role::RoleService, theatre::TheatreService, user::UserService,
};
use util::{get_connection_pool, hash_mock_passwords};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
use vars::{gmail_password, gmail_user};

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

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(Credentials::new(gmail_user().unwrap(), gmail_password().unwrap()))
        .build();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(movie_service.clone()))
            .app_data(web::Data::new(theatre_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(bridge_role_service.clone()))
            .app_data(web::Data::new(role_service.clone()))
            .app_data(web::Data::new(language_service.clone()))
            .app_data(web::Data::new(mailer.clone()))
            .service(web::scope("/api/v1").configure(handlers::config))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", doc::ApiDoc::openapi()),
            )
            .route("/", web::get().to(root_response))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
