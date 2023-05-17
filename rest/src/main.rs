use core::panic;
mod user;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::mssql::MssqlPoolOptions;

async fn root_response() -> impl Responder {
    HttpResponse::Ok().body("*wind noises*")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be present in environment vars");

    let pool = MssqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await;

    let pool = match pool {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    };

    HttpServer::new(|| App::new().route("/", web::get().to(root_response)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
