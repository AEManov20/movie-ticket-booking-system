use crate::model::{Ticket, MovieReview, PartialUser, ExtendedMovieReview};

use super::*;

// TODO!
#[utoipa::path(context_path = "/api/v1/user")]
#[get("/@me")]
async fn get_self_user() -> Result<User> {
    todo!();
}

#[utoipa::path(context_path = "/api/v1/user")]
#[get("/@me/tickets")]
async fn get_self_tickets() -> Result<Ticket> {
    todo!();
}

async fn get_self_reviews() -> Result<ExtendedMovieReview> {
    todo!();
}

#[utoipa::path(context_path = "/api/v1/user")]
#[get("/{id}")]
async fn get_partial_user() -> Result<PartialUser> {
    todo!();
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
    );
}