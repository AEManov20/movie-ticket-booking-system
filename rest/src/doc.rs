use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi
};

use crate::{handlers::auth::EmailVerificationQuery, services::SortBy};
use crate::handlers::movie::{MovieReviewQuery, MovieQuery};
use crate::handlers::role::BridgeRoleQuery;
use crate::services::user::LoginResponse;

use super::*;
use model::*;

use handlers::theatre::role::{UserRoleForm, RoleUpdateAction};

struct AuthAddon;

impl Modify for AuthAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("authorization"))),
        )
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::role::get_all_roles,
        handlers::role::query_bridge_roles,
    ),
    components(
        schemas(SortBy, LoginResponse, Language, MovieReview, Theatre, Movie, UserTheatreRole, Hall, TheatreScreening, TheatreScreeningEvent, TicketType, FormUser, FormTheatreScreening, FormHall, FormTheatre, FormMovie, FormTicketType, FormMovieReview, UserRoleForm, RoleUpdateAction, LoginUser, EmailVerificationQuery, MovieQuery, BridgeRoleQuery),
    ),
    modifiers(&AuthAddon)
)]
pub struct RoleApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::theatre::new_theatre,
        handlers::theatre::get_theatre,
        handlers::theatre::update_theatre,
        handlers::theatre::delete_theatre,
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
    ),
    components(
        schemas(SortBy, LoginResponse, Language, MovieReview, Theatre, Movie, UserTheatreRole, Hall, TheatreScreening, TheatreScreeningEvent, TicketType, FormUser, FormTheatreScreening, FormHall, FormTheatre, FormMovie, FormTicketType, FormMovieReview, UserRoleForm, RoleUpdateAction, LoginUser, EmailVerificationQuery, MovieQuery, BridgeRoleQuery),
    ),
    modifiers(&AuthAddon)
)]
pub struct TheatreApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::movie::submit_new_review,
        handlers::movie::get_review_by_id,
        handlers::movie::delete_review_by_id,
        handlers::movie::get_reviews,
        handlers::movie::get_theatres_by_movie_id,
        handlers::movie::query_movies,
        handlers::movie::get_movie_by_id,
        handlers::movie::delete_movie_by_id,
        handlers::movie::create_movie,
    ),
    components(
        schemas(SortBy, LoginResponse, Language, MovieReview, Theatre, Movie, UserTheatreRole, Hall, TheatreScreening, TheatreScreeningEvent, TicketType, FormUser, FormTheatreScreening, FormHall, FormTheatre, FormMovie, FormTicketType, FormMovieReview, UserRoleForm, RoleUpdateAction, LoginUser, EmailVerificationQuery, MovieQuery, BridgeRoleQuery),
    ),
    modifiers(&AuthAddon)
)]
pub struct MovieApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::language::get_all_languages,
        handlers::language::get_language,
    ),
    components(
        schemas(SortBy, LoginResponse, Language, MovieReview, Theatre, Movie, UserTheatreRole, Hall, TheatreScreening, TheatreScreeningEvent, TicketType, FormUser, FormTheatreScreening, FormHall, FormTheatre, FormMovie, FormTicketType, FormMovieReview, UserRoleForm, RoleUpdateAction, LoginUser, EmailVerificationQuery, MovieQuery, BridgeRoleQuery),
    ),
    modifiers(&AuthAddon)
)]
pub struct LanguageApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::login_user,
        handlers::auth::register_user,
        handlers::auth::verify_email,
    ),
    components(
        schemas(SortBy, LoginResponse, Language, MovieReview, Theatre, Movie, UserTheatreRole, Hall, TheatreScreening, TheatreScreeningEvent, TicketType, FormUser, FormTheatreScreening, FormHall, FormTheatre, FormMovie, FormTicketType, FormMovieReview, UserRoleForm, RoleUpdateAction, LoginUser, EmailVerificationQuery, MovieQuery, BridgeRoleQuery),
    ),
    modifiers(&AuthAddon)
)]
pub struct AuthApiDoc;
