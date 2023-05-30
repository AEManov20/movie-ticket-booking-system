use actix_web::dev::Payload;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{http, FromRequest, HttpRequest};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::{Date, Json, Timestamptz};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use validator::Validate;

use crate::schema::*;
use crate::util::JWT_ALGO;
use crate::vars::jwt_user_secret;

#[derive(Identifiable, Queryable, Debug, Clone, AsChangeset)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub is_super_user: bool,
    pub is_activated: bool,
    pub is_deleted: bool,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct SlimUser {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub is_activated: bool,
    pub is_deleted: bool,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct FormUser {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub username: String,
    #[validate(length(min = 12))]
    pub password: String,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct LoginUser {
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 12))]
    pub password: String,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(User, foreign_key = owner_user_id))]
#[diesel(belongs_to(TheatreMovie))]
#[diesel(belongs_to(TicketType))]
pub struct Ticket {
    pub id: uuid::Uuid,
    pub owner_user_id: uuid::Uuid,
    pub theatre_movie_id: uuid::Uuid,
    pub ticket_type_id: uuid::Uuid,
    pub issuer_user_id: uuid::Uuid,
    pub seat_row: i32,
    pub seat_column: i32,
    pub issued_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub used: bool,
}

pub struct FormTicket {
    pub theatre_movie_id: uuid::Uuid,
    pub ticket_type_id: uuid::Uuid,
    pub issuer_user_id: uuid::Uuid,
    pub seat_row: i32,
    pub seat_column: i32,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = tickets)]
pub struct CreateTicket {
    pub owner_user_id: uuid::Uuid,
    pub theatre_movie_id: uuid::Uuid,
    pub ticket_type_id: uuid::Uuid,
    pub issuer_user_id: uuid::Uuid,
    pub seat_row: i32,
    pub seat_column: i32,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(Hall))]
#[diesel(belongs_to(Movie))]
#[diesel(belongs_to(Theatre))]
pub struct TheatreMovie {
    pub id: uuid::Uuid,
    pub movie_id: uuid::Uuid,
    pub theatre_id: uuid::Uuid,
    pub hall_id: uuid::Uuid,
    pub subtitles_language: Option<String>,
    pub audio_language: String,
    pub starting_time: chrono::NaiveDateTime,
    pub status: i32,
}

#[derive(Insertable, Deserialize, AsChangeset, Validate)]
#[diesel(table_name = theatre_movies)]
pub struct FormTheatreMovie {
    pub movie_id: uuid::Uuid,
    pub theatre_id: uuid::Uuid,
    pub hall_id: uuid::Uuid,
    #[validate(length(min = 4))]
    pub subtitles_language: Option<String>,
    #[validate(length(min = 4))]
    pub audio_language: String,
    pub starting_time: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(Theatre))]
pub struct Hall {
    pub id: uuid::Uuid,
    pub number: i32,
    pub theatre_id: uuid::Uuid,
    pub seat_data: serde_json::Value,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = halls)]
pub struct FormHall {
    pub number: i32,
    pub theatre_id: uuid::Uuid,
    pub seat_data: serde_json::Value,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset)]
pub struct Theatre {
    pub id: uuid::Uuid,
    pub name: String,
    pub location_lat: f64,
    pub location_lon: f64,
    pub is_deleted: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = theatres)]
pub struct FormTheatre {
    pub name: String,
    pub location_lat: f64,
    pub location_lon: f64,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset)]
pub struct Movie {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub genre: String,
    pub release_date: chrono::NaiveDate,
    pub length: f64,
    pub votes: i32,
    pub imdb_link: Option<String>,
    pub is_deleted: bool,
}

#[derive(Insertable, Deserialize, AsChangeset, Clone, Validate)]
#[diesel(table_name = movies)]
pub struct FormMovie {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: String,
    pub genre: String,
    pub release_date: chrono::NaiveDate,
    pub length: f64,
    #[validate(url)]
    pub imdb_link: Option<String>,
}

#[derive(Identifiable, Associations, Queryable, Serialize, Debug, Clone, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = author_user_id))]
#[diesel(belongs_to(Movie))]
pub struct MovieReview {
    pub id: uuid::Uuid,
    pub author_user_id: uuid::Uuid,
    pub movie_id: uuid::Uuid,
    pub content: Option<String>,
    pub rating: f64,
    pub created_at: chrono::NaiveDateTime,
    pub votes: i32,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = movie_reviews)]
pub struct CreateMovieReview {
    pub author_user_id: uuid::Uuid,
    pub movie_id: uuid::Uuid,
    pub content: Option<String>,
    pub rating: f64,
}

pub struct FormMovieReview {
    pub movie_id: uuid::Uuid,
    pub content: Option<String>,
    pub rating: f64,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(Theatre))]
#[diesel(belongs_to(User))]
#[diesel(primary_key(user_id, theatre_id))]
pub struct TheatrePermission {
    pub user_id: uuid::Uuid,
    pub theatre_id: uuid::Uuid,
    pub can_manage_users: bool,
    pub can_manage_movies: bool,
    pub can_check_tickets: bool,
    pub can_manage_tickets: bool,
    pub is_theatre_owner: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = theatre_permissions)]
pub struct FormTheatrePermission {
    pub user_id: uuid::Uuid,
    pub theatre_id: uuid::Uuid,
    pub can_manage_users: bool,
    pub can_manage_movies: bool,
    pub can_check_tickets: bool,
    pub can_manage_tickets: bool,
    pub is_theatre_owner: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = theatre_permissions)]
pub struct UpdateTheatrePermission {
    pub can_manage_users: bool,
    pub can_manage_movies: bool,
    pub can_check_tickets: bool,
    pub can_manage_tickets: bool,
    pub is_theatre_owner: bool,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(Theatre))]
pub struct TicketType {
    id: uuid::Uuid,
    type_: String,
    // a.k.a. template_type
    movie_type: String,
    description: Option<String>,
    theatre_id: uuid::Uuid,
    currency: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "v")]
pub enum JwtType {
    // id (Ticket)
    Ticket(uuid::Uuid),
    // id (User)
    Email(uuid::Uuid),
    // id (User)
    User(uuid::Uuid),
    // id (User)
    Refresh(uuid::Uuid),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub dat: JwtType,
    pub sub: uuid::Uuid,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = ticket_types)]
pub struct FormTicketType {
    #[serde(alias = "type")]
    type_: String,
    // a.k.a. template_type
    movie_type: String,
    description: String,
    theatre_id: uuid::Uuid,
    currency: String,
    price: f64,
}

impl From<User> for SlimUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            username: value.username,
            created_at: value.created_at,
            is_activated: value.is_activated,
            is_deleted: value.is_deleted,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum AuthErrorType {
    NoToken,
    InvalidToken,
    TokenExpired,
    UserDoesNotExist,
    Other,
}

impl FromRequest for JwtClaims {
    type Error = crate::handlers::ErrorType;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        use crate::handlers::{ErrorType};

        let Some(token) = req.headers().get(http::header::AUTHORIZATION) else {
            return ready(Err(ErrorType::NoAuth))
        };

        let Ok(token) = token.to_str() else {
            return ready(Err(ErrorType::ServerError));
        };

        ready(
            match decode::<JwtClaims>(
                token,
                &DecodingKey::from_secret(jwt_user_secret().as_ref()),
                &Validation::new(*JWT_ALGO),
            ) {
                Ok(c) => {
                    if Utc::now().timestamp() > c.claims.exp {
                        return ready(Err(ErrorType::Expired));
                    }

                    if let JwtType::User(_) = c.claims.dat {
                        Ok(c.claims)
                    } else {
                        Err(ErrorType::Invalid)
                    }
                }
                Err(_) => Err(ErrorType::Invalid),
            },
        )
    }
}