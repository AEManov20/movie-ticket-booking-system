use utoipa::OpenApi;

use super::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::theatre::hall::get_halls,
        handlers::theatre::hall::create_hall,
        handlers::theatre::hall::delete_hall,
        handlers::theatre::role::get_all_roles,
        handlers::theatre::role::update_roles_batch,
        handlers::theatre::screening::get_timeline,
        handlers::theatre::screening::get_theatre_screening,
        handlers::theatre::screening::update_theatre_screening,
        handlers::theatre::screening::delete_theatre_screening,
        handlers::theatre::screening::create_theatre_screening,
        handlers::theatre::ticket_type::get_all_ticket_types,
        handlers::theatre::ticket_type::create_ticket_type,
        handlers::theatre::ticket_type::delete_ticket_type,
        handlers::auth::login_user,
        handlers::auth::register_user,
        handlers::auth::verify_email,
        handlers::language::get_all_languages,
        handlers::language::get_language,
        handlers::movie::submit_new_review,
        handlers::movie::get_review_by_id,
        handlers::movie::delete_review_by_id,
        handlers::movie::get_reviews,
        handlers::movie::get_theatres_by_movie_id,
        handlers::movie::query_movies,
        handlers::movie::get_movie_by_id,
        handlers::movie::delete_movie_by_id,
        handlers::movie::create_movie,
        handlers::role::get_all_roles,
        handlers::role::query_bridge_roles,
    )
)]
pub struct ApiDoc;
