use actix_web::dev::Payload;
use actix_web::{http, FromRequest, HttpRequest};
use chrono::Utc;
use diesel::prelude::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use utoipa::{IntoParams, ToResponse, ToSchema};
use validator::Validate;

use crate::schema::*;
use crate::util::JWT_ALGO;
use crate::vars::jwt_user_secret;

#[derive(
    Selectable, Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, AsChangeset, ToSchema,
)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub is_super_user: bool,
    #[serde(skip)]
    pub is_activated: bool,
    #[serde(skip)]
    pub is_deleted: bool,
}

#[derive(Selectable, Identifiable, Queryable, Debug, Serialize, Clone, AsChangeset, ToSchema)]
#[diesel(table_name = users)]
pub struct PartialUser {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub is_super_user: bool
}

#[derive(Deserialize, Debug, Clone, Validate, ToSchema, IntoParams)]
pub struct FormUser {
    #[schema(example = "John")]
    #[validate(length(min = 1, max = 50))]
    pub first_name: String,
    #[schema(example = "Doe")]
    #[validate(length(min = 1, max = 50))]
    pub last_name: String,
    #[schema(example = "john.doe@example.com")]
    #[validate(email, length(max = 150))]
    pub email: String,
    #[schema(example = "john.doe")]
    #[validate(length(min = 8, max = 50))]
    pub username: String,
    #[schema(example = "password_123_do_not_use")]
    #[validate(length(min = 12))]
    pub password: String,
}

#[derive(Deserialize, Debug, Clone, Validate, IntoParams, ToSchema)]
pub struct LoginUser {
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 12))]
    pub password: String,
}

#[derive(
    Selectable, Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations,
)]
#[diesel(belongs_to(User, foreign_key = owner_user_id))]
#[diesel(belongs_to(TheatreScreening))]
#[diesel(belongs_to(TicketType))]
pub struct Ticket {
    pub id: uuid::Uuid,
    pub owner_user_id: uuid::Uuid,
    pub theatre_screening_id: uuid::Uuid,
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
    pub theatre_screening_id: uuid::Uuid,
    pub ticket_type_id: uuid::Uuid,
    pub issuer_user_id: uuid::Uuid,
    pub seat_row: i32,
    pub seat_column: i32,
}

#[derive(
    Selectable,
    Identifiable,
    Queryable,
    Serialize,
    Debug,
    Clone,
    AsChangeset,
    Associations,
    ToSchema,
)]
#[diesel(belongs_to(Hall))]
#[diesel(belongs_to(Movie))]
#[diesel(belongs_to(Theatre))]
pub struct TheatreScreening {
    pub id: uuid::Uuid,
    pub movie_id: uuid::Uuid,
    pub theatre_id: uuid::Uuid,
    pub hall_id: uuid::Uuid,
    pub subtitles_language_id: Option<uuid::Uuid>,
    pub audio_language_id: uuid::Uuid,
    pub starting_time: chrono::NaiveDateTime,
    pub is_3d: bool,
    pub status: i32,
    #[serde(skip)]
    pub is_deleted: bool,
}

#[derive(Insertable, Deserialize, AsChangeset, Validate, ToSchema)]
#[diesel(table_name = theatre_screenings)]
pub struct FormTheatreScreening {
    pub movie_id: uuid::Uuid,
    pub theatre_id: uuid::Uuid,
    pub hall_id: uuid::Uuid,
    pub subtitles_language_id: Option<uuid::Uuid>,
    pub audio_language_id: uuid::Uuid,
    pub starting_time: chrono::NaiveDateTime,
    pub is_3d: Option<bool>,
}

#[derive(Serialize, Queryable, ToSchema)]
pub struct TheatreScreeningEvent {
    pub movie_id: uuid::Uuid,
    pub theatre_movie_id: uuid::Uuid,
    pub starting_time: chrono::NaiveDateTime,
    pub length: f64,
    pub movie_name: String,
}

#[derive(
    Selectable,
    Identifiable,
    Queryable,
    Serialize,
    Debug,
    Clone,
    AsChangeset,
    Associations,
    ToSchema,
)]
#[diesel(belongs_to(Theatre))]
pub struct Hall {
    pub id: uuid::Uuid,
    pub theatre_id: uuid::Uuid,
    pub name: String,
    pub seat_data: serde_json::Value,
    #[serde(skip)]
    pub is_deleted: bool,
}

#[derive(Insertable, Deserialize, AsChangeset, Validate, ToSchema)]
#[diesel(table_name = halls)]
pub struct FormHall {
    #[validate(length(max = 50))]
    #[schema(example = "Apollo 1")]
    pub name: String,
    pub theatre_id: uuid::Uuid,
    pub seat_data: serde_json::Value,
}

#[derive(Selectable, Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, ToSchema)]
pub struct Theatre {
    pub id: uuid::Uuid,
    pub name: String,
    pub location_lat: f64,
    pub location_lon: f64,
    #[serde(skip)]
    pub is_deleted: bool,
}

#[derive(Insertable, Deserialize, AsChangeset, Validate, ToSchema)]
#[diesel(table_name = theatres)]
pub struct FormTheatre {
    #[schema(example = "September")]
    #[validate(length(max = 50))]
    pub name: String,
    #[schema(example = 42.49488)]
    #[validate(range(min = -180., max = 180.))]
    pub location_lon: f64,
    #[schema(example = 27.47278)]
    #[validate(range(min = -90., max = 90.))]
    pub location_lat: f64,
}

#[derive(Selectable, Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, ToSchema)]
pub struct Movie {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub genre: String,
    pub release_date: chrono::NaiveDate,
    pub length: f64,
    pub imdb_link: Option<String>,
    #[serde(skip)]
    pub is_deleted: bool,
}

#[derive(Selectable, Identifiable, Queryable, Debug, Serialize, Clone, AsChangeset, ToSchema)]
#[diesel(table_name = movies)]
pub struct PartialMovie {
    pub id: uuid::Uuid,
    pub name: String,
    pub genre: String,
    pub release_date: chrono::NaiveDate,
}

#[derive(Insertable, Deserialize, AsChangeset, Clone, Validate, ToSchema)]
#[diesel(table_name = movies)]
pub struct FormMovie {
    #[schema(example = "Harry Potter")]
    #[validate(length(min = 1))]
    pub name: String,
    #[schema(example = "lorem ipsum doret")]
    #[validate(length(max = 65535))]
    pub description: String,
    #[schema(example = "Action|Romance|Fantasy")]
    #[validate(length(max = 250))]
    pub genre: String,
    #[schema(example = json!(chrono::NaiveDate::from_isoywd_opt(2009, 41, chrono::Weekday::Thu)))]
    pub release_date: chrono::NaiveDate,
    #[schema(example = 161.)]
    pub length: f64,
    #[validate(url, length(max = 250))]
    pub imdb_link: Option<String>,
}

#[derive(
    Selectable,
    Identifiable,
    Associations,
    Queryable,
    Serialize,
    Debug,
    Clone,
    AsChangeset,
    ToSchema,
)]
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

#[derive(Selectable, Identifiable, Queryable, Debug, Serialize, Clone, AsChangeset, ToSchema)]
#[diesel(table_name = movie_reviews)]
pub struct PartialMovieReview {
    pub id: uuid::Uuid,
    pub content: Option<String>,
    pub rating: f64,
    pub created_at: chrono::NaiveDateTime,
    pub votes: i32,
}

#[derive(Serialize, Queryable, ToSchema)]
#[diesel(table_name = movie_reviews)]
pub struct ExtendedUserReview {
    user: PartialUser,
    review: PartialMovieReview,
}

#[derive(Serialize, Queryable, ToSchema)]
#[diesel(table_name = movie_reviews)]
pub struct ExtendedMovieReview {
    movie: PartialMovie,
    review: PartialMovieReview,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = movie_reviews)]
pub struct CreateMovieReview {
    pub author_user_id: uuid::Uuid,
    pub movie_id: uuid::Uuid,
    pub content: Option<String>,
    pub rating: f64,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct FormMovieReview {
    pub movie_id: uuid::Uuid,
    #[schema(example = "This movie is the best 11/10.")]
    #[validate(length(max = 2500))]
    pub content: Option<String>,
    #[schema(example = 0.95)]
    #[validate(range(min = 0., max = 1.))]
    pub rating: f64,
}

#[derive(
    Selectable,
    Identifiable,
    Queryable,
    Serialize,
    Debug,
    Clone,
    AsChangeset,
    Associations,
    ToSchema,
)]
#[diesel(belongs_to(Theatre))]
pub struct TicketType {
    pub id: uuid::Uuid,
    #[serde(alias = "type")]
    #[serde(rename(serialize = "type"))]
    pub type_: String,
    pub description: Option<String>,
    pub theatre_id: uuid::Uuid,
    pub currency: String,
    pub price: f64,
    #[serde(skip)]
    pub is_deleted: bool,
}

#[derive(
    Selectable,
    Identifiable,
    Insertable,
    Queryable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Associations,
    ToSchema,
)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Theatre))]
#[diesel(belongs_to(TheatreRole, foreign_key = role_id))]
#[diesel(primary_key(user_id, role_id, theatre_id))]
#[diesel(table_name = users_theatre_roles)]
pub struct UserTheatreRole {
    pub user_id: uuid::Uuid,
    pub role_id: uuid::Uuid,
    pub theatre_id: uuid::Uuid,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset)]
pub struct TheatreRole {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug)]
pub enum Role {
    TheatreOwner,
    TicketManager,
    TicketChecker,
    UserManager,
    ScreeningsManager,
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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub dat: JwtType,
    pub sub: uuid::Uuid,
    pub iat: i64,
    pub exp: i64,
}

#[derive(
    Selectable, Identifiable, Insertable, Queryable, Serialize, Deserialize, Debug, Clone, ToSchema,
)]
pub struct Language {
    pub id: uuid::Uuid,
    pub code: String,
    pub name: String,
}

#[derive(Insertable, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = ticket_types)]
pub struct FormTicketType {
    #[serde(alias = "type")]
    #[serde(rename(serialize = "type"))]
    pub type_: String,
    pub description: String,
    pub theatre_id: uuid::Uuid,
    pub currency: String,
    pub price: f64,
}

impl Role {
    pub fn try_from_str(value: &str) -> Option<Self> {
        match value {
            "TheatreOwner" => Some(Role::TheatreOwner),
            "TicketManager" => Some(Role::TicketManager),
            "TicketChecker" => Some(Role::TicketChecker),
            "ScreeningsManager" => Some(Role::ScreeningsManager),
            "UserManager" => Some(Role::UserManager),
            _ => None,
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromRequest for JwtClaims {
    type Error = crate::handlers::ErrorType;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        use crate::handlers::ErrorType;

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
                        return ready(Err(ErrorType::NoAuth));
                    }

                    if let JwtType::User(_) = c.claims.dat {
                        Ok(c.claims)
                    } else {
                        Err(ErrorType::NoAuth)
                    }
                }
                Err(_) => Err(ErrorType::NoAuth),
            },
        )
    }
}

impl From<User> for PartialUser {
    fn from(value: User) -> Self {
        PartialUser {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            username: value.username,
            is_super_user: value.is_super_user
        }
    }
}
