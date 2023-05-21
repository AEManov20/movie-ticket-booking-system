use std::io::{Write, Read};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use diesel::deserialize::FromSql;
use diesel::sql_types::{Timestamptz, Json, Date};
use diesel::prelude::*;
use diesel::serialize::ToSql;
use serde::{Serialize, Deserialize};
use crate::services::user::UserResource;

use crate::schema::*;

#[derive(Identifiable, Queryable, Debug, Clone, AsChangeset)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: Option<String>,
    pub is_super_user: bool,
    pub is_activated: bool,
    pub is_deleted: bool,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct SlimUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub is_activated: bool,
    pub is_deleted: bool
}

#[derive(Deserialize, Debug, Clone)]
pub struct FormUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(User))]
pub struct EmailConfirmation {
    pub id: i32,
    pub key: String,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(User, foreign_key = owner_user_id))]
#[diesel(belongs_to(TheatreMovie))]
#[diesel(belongs_to(TicketType))]
pub struct Ticket {
    pub id: i32,
    pub owner_user_id: i32,
    pub theatre_movie_id: i32,
    pub ticket_type_id: i32,
    pub issuer_user_id: Option<i32>,
    pub seat_row: i32,
    pub seat_column: i32,
    pub expires_at: chrono::NaiveDateTime,
    pub used: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = tickets)]
pub struct FormTicket {
    pub owner_user_id: i32,
    pub theatre_movie_id: i32,
    pub ticket_type_id: i32,
    pub issuer_user_id: Option<i32>,
    pub seat_row: i32,
    pub seat_column: i32,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(Hall))]
pub struct TheatreMovie {
    pub id: i32,
    pub movie_id: i32,
    pub hall_id: i32,
    pub subtitles_language: Option<String>,
    pub audio_language: String,
    pub starting_time: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = theatre_movies)]
pub struct FormTheatreMovie {
    pub movie_id: i32,
    pub hall_id: i32,
    pub subtitles_language: Option<String>,
    pub audio_language: String,
    pub starting_time: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(Theatre))]
pub struct Hall {
    pub id: i32,
    pub number: i32,
    pub theatre_id: i32,
    pub seat_data: serde_json::Value,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = halls)]
pub struct FormHall {
    pub number: i32,
    pub theatre_id: i32,
    pub seat_data: serde_json::Value,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset)]
pub struct Theatre {
    pub id: i32,
    pub name: String,
    pub location_lat: f64,
    pub location_lon: f64,
    pub is_deleted: bool
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
    pub id: i32,
    pub name: String,
    pub description: String,
    pub genre: String,
    pub release_date: chrono::NaiveDate,
    pub length: f64,
    pub imdb_link: Option<String>,
    pub is_deleted: bool
}

#[derive(Insertable, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = movies)]
pub struct FormMovie {
    pub name: String,
    pub description: String,
    pub genre: String,
    pub release_date: chrono::NaiveDate,
    pub length: f64,
    pub imdb_link: Option<String>,
}

#[derive(Identifiable, Associations, Queryable, Serialize, Debug, Clone, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = author_user_id))]
#[diesel(belongs_to(Movie))]
pub struct MovieReview {
    pub id: i32,
    pub author_user_id: i32,
    pub movie_id: i32,
    pub content: Option<String>,
    pub rating: f64,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = movie_reviews)]
pub struct FormMovieReview {
    pub author_user_id: i32,
    pub movie_id: i32,
    pub content: Option<String>,
    pub rating: f64,
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset, Associations)]
#[diesel(belongs_to(Theatre))]
#[diesel(belongs_to(User))]
pub struct TheatrePermission {
    pub id: i32,
    pub user_id: i32,
    pub theatre_id: i32,
    pub can_manage_users: bool,
    pub can_manage_movies: bool,
    pub can_check_tickets: bool,
    pub can_manage_tickets: bool,
    pub is_theatre_owner: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = theatre_permissions)]
pub struct FormTheatrePermission {
    pub user_id: i32,
    pub theatre_id: i32,
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
    pub is_theatre_owner: bool
}

#[derive(Identifiable, Queryable, Serialize, Debug, Clone, AsChangeset)]
#[diesel(belongs_to(Theatre))]
pub struct TicketType {
    id: i32,
    type_: String,
    // a.k.a. template_type
    movie_type: String,
    description: String,
    theatre_id: i32,
    currency: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum JwtType {
    // id (Ticket)
    Ticket(i32),
    // id (EmailConfirmation)
    Email(i32),
    // id (User)
    User(i32)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub dat: JwtType,
    pub sub: i32,
    pub iss: Option<i32>,
    pub iat: i64,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = ticket_types)]
pub struct FormTicketType {
    type_: String,
    // a.k.a. template_type
    movie_type: String,
    description: String,
    theatre_id: i32,
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
            is_activated: value.is_activated,
            is_deleted: value.is_deleted
        }
    }
}
