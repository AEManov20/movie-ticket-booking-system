use crate::util::JWT_ALGO;
use deadpool_diesel::postgres::Pool;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use jsonwebtoken::Validation;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use rayon::prelude::*;
use serde::Serialize;
use utoipa::ToResponse;

use super::DatabaseError;
use crate::model::*;
use crate::password;
use crate::vars::{jwt_email_secret, jwt_ticket_secret, jwt_user_secret};

pub const EMAIL_CONFIRMATION_TOKEN_EXPIRY_DAYS: i64 = 1;
pub const USER_TOKEN_EXPIRY_DAYS: i64 = 2;

#[derive(Clone)]
pub struct UserService {
    pool: Pool,
}

impl UserService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: FormUser) -> Result<UserResource, DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;

        let hash = password::hash(user.password.as_bytes())?;

        if !password::verify(user.password.as_bytes(), &hash) {
            return Err(DatabaseError::Other("Hash failed verification".to_string()));
        }

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(users)
                    .values((
                        first_name.eq(user.first_name.clone()),
                        last_name.eq(user.last_name.clone()),
                        email.eq(user.email.clone()),
                        username.eq(user.username.clone()),
                        password_hash.eq(hash),
                    ))
                    .returning(User::as_returning())
                    .get_result::<User>(conn)
            })
            .await??;

        Ok(UserResource::new(result, self.pool.clone()))
    }

    pub async fn get_by_email(
        &self,
        email_: String,
    ) -> Result<Option<UserResource>, DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;

        let result = conn
            .interact(|conn| {
                users
                    .limit(1)
                    .filter(is_deleted.eq(false))
                    .filter(email.eq(email_))
                    .load::<User>(conn)
            })
            .await??
            .first()
            .cloned();

        match result {
            Some(user) => Ok(Some(UserResource::new(user, self.pool.clone()))),
            None => Ok(None),
        }
    }

    pub async fn get_by_email_or_username(
        &self,
        email_: Option<String>,
        username_: Option<String>,
    ) -> Result<Option<UserResource>, DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;

        let result = conn
            .interact(|conn| {
                let mut query = users.limit(1).filter(is_deleted.eq(false)).into_boxed();

                if let Some(email_) = email_ {
                    query = query.or_filter(email.eq(email_));
                }

                if let Some(username_) = username_ {
                    query = query.or_filter(username.eq(username_));
                }

                query.load::<User>(conn)
            })
            .await??
            .first()
            .cloned();

        match result {
            Some(user) => Ok(Some(UserResource::new(user, self.pool.clone()))),
            None => Ok(None),
        }
    }

    pub async fn get_by_id(&self, id_: uuid::Uuid) -> Result<Option<UserResource>, DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;

        let result = conn
            .interact(move |conn| {
                users
                    .filter(is_deleted.eq(false))
                    .filter(id.eq(id_))
                    .limit(1)
                    .load::<User>(conn)
            })
            .await??
            .first()
            .cloned();

        match result {
            Some(user) => Ok(Some(UserResource::new(user, self.pool.clone()))),
            None => Ok(None),
        }
    }

    pub async fn delete(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::users::dsl::*;

        self.pool
            .get()
            .await?
            .interact(move |conn| {
                diesel::update(users)
                    .filter(is_deleted.eq(false))
                    .filter(id.eq(id_))
                    .set(is_deleted.eq(true))
                    .execute(conn)
            })
            .await??;

        Ok(())
    }
}

#[derive(Serialize, ToResponse)]
pub struct LoginResponse {
    token: String,
}

#[derive(Clone)]
pub struct UserResource {
    user: User,
    pool: Pool,
}

impl UserResource {
    fn new(user: User, pool: Pool) -> Self {
        Self { user, pool }
    }

    pub fn create_jwt(&self) -> Result<LoginResponse, DatabaseError> {
        let token = encode(
            &Header::new(*JWT_ALGO),
            &JwtClaims {
                dat: JwtType::User(self.user.id),
                sub: self.user.id,
                iat: (chrono::Utc::now()).timestamp(),
                exp: (chrono::Utc::now() + chrono::Duration::days(USER_TOKEN_EXPIRY_DAYS))
                    .timestamp(),
            },
            &EncodingKey::from_secret(jwt_user_secret().as_bytes()),
        )?;

        Ok(LoginResponse { token })
    }

    pub fn verify_user_jwt(user_jwt: &str) -> Option<JwtClaims> {
        let data = decode::<JwtClaims>(
            user_jwt,
            &DecodingKey::from_secret(jwt_user_secret().as_bytes()),
            &Validation::new(*JWT_ALGO),
        );

        if let Ok(data) = data {
            Some(data.claims)
        } else {
            None
        }
    }

    pub fn create_email_jwt(&self) -> Result<String, DatabaseError> {
        Ok(jsonwebtoken::encode(
            &Header::new(*JWT_ALGO),
            &JwtClaims {
                dat: JwtType::Email(self.user.id),
                sub: self.user.id,
                iat: chrono::Utc::now().timestamp(),
                exp: (chrono::Utc::now()
                    + chrono::Duration::days(EMAIL_CONFIRMATION_TOKEN_EXPIRY_DAYS))
                .timestamp(),
            },
            &EncodingKey::from_secret(jwt_email_secret().as_ref()),
        )?)
    }

    pub fn verify_email_jwt(email_jwt: &str) -> Result<JwtClaims, DatabaseError> {
        Ok(decode::<JwtClaims>(
            email_jwt,
            &DecodingKey::from_secret(jwt_email_secret().as_bytes()),
            &Validation::new(*JWT_ALGO),
        )?
        .claims)
    }

    pub async fn activate(&self) -> Result<(), DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;
        let user = self.user.clone();

        conn.interact(move |conn| {
            diesel::update(users)
                .filter(id.eq(user.id))
                .set(is_activated.eq(true))
                .execute(conn)
        })
        .await??;

        Ok(())
    }

    pub async fn update_user(&mut self, new_user: FormUser) -> Result<(), DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;
        let user = self.user.clone();

        let result = conn
            .interact(move |conn| {
                diesel::update(users)
                    .filter(id.eq(user.id))
                    .set((
                        first_name.eq(new_user.first_name.clone()),
                        last_name.eq(new_user.last_name.clone()),
                        email.eq(new_user.email.clone()),
                        username.eq(new_user.username),
                    ))
                    .returning(User::as_returning())
                    .get_result(conn)
            })
            .await??;

        self.user = result;

        Ok(())
    }

    pub async fn update_password(
        &mut self,
        new_password: String,
    ) -> Result<(), DatabaseError> {
        let conn = self.pool.get().await?;
        let user = self.user.clone();

        fn insert_password(
            id_: uuid::Uuid,
            pass: &[u8],
            conn: &mut PgConnection,
        ) -> QueryResult<User> {
            use crate::schema::users::dsl::*;

            let hash = password::hash(pass);

            match hash {
                Ok(hash) => {
                    if !password::verify(pass, &hash) {
                        return Err(NotFound);
                    }

                    Ok(diesel::update(users)
                        .filter(id.eq(id_))
                        .set(password_hash.eq(hash))
                        .returning(User::as_returning())
                        .get_result(conn)?)
                }
                Err(_) => Err(NotFound),
            }
        }

        let Some(ref hash) = self.user.password_hash else {
            let res = conn.interact(move |conn| insert_password(user.id, new_password.as_bytes(), conn)).await??;
            self.user.password_hash = res.password_hash;
            return Ok(())
        };

        conn.interact(move |conn| insert_password(user.id, new_password.as_bytes(), conn))
            .await??;
        Ok(())
    }

    pub async fn get_tickets(&self) -> Result<Vec<TicketResource>, DatabaseError> {
        let conn = self.pool.get().await?;
        let cloned_user = self.user.clone();

        Ok(conn
            .interact(move |conn| Ticket::belonging_to(&cloned_user).load(conn))
            .await??
            .par_iter()
            .map(|e: &Ticket| TicketResource::new(e.clone(), self.pool.clone()))
            .collect::<Vec<_>>())
    }

    pub async fn create_ticket(
        &self,
        issuer_user_id: uuid::Uuid,
        theatre_movie_id: uuid::Uuid,
        ticket_type_id: uuid::Uuid,
        seat_row: i32,
        seat_column: i32,
    ) -> Result<TicketResource, DatabaseError> {
        let conn = self.pool.get().await?;

        let ticket = CreateTicket {
            owner_user_id: self.user.id,
            theatre_screening_id: theatre_movie_id,
            ticket_type_id,
            issuer_user_id,
            seat_row,
            seat_column,
        };

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(Ticket::table())
                    .values(ticket)
                    .returning(Ticket::as_returning())
                    .get_result(conn)
            })
            .await??;
        
        Ok(TicketResource::new(result, self.pool.clone()))
    }

    pub async fn delete_ticket(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::tickets::dsl::*;

        let conn = self.pool.get().await?;

        conn.interact(move |conn| diesel::delete(tickets).filter(id.eq(id_)).execute(conn))
            .await??;

        Ok(())
    }

    pub async fn get_reviews(&self) -> Result<Vec<MovieReview>, DatabaseError> {
        let conn = self.pool.get().await?;
        let cloned_user = self.user.clone();

        Ok(conn
            .interact(move |conn| MovieReview::belonging_to(&cloned_user).load(conn))
            .await??)
    }

    pub async fn create_review(
        &self,
        content: Option<String>,
        rating: f64,
        movie_id: uuid::Uuid,
    ) -> Result<MovieReview, DatabaseError> {
        let conn = self.pool.get().await?;

        let review = CreateMovieReview {
            author_user_id: self.user.id,
            content,
            rating,
            movie_id,
        };

        Ok(conn
            .interact(move |conn| {
                diesel::insert_into(MovieReview::table())
                    .values(&review)
                    .returning(MovieReview::as_returning())
                    .get_result(conn)
            })
            .await??)
    }

    pub async fn update_review(
        &self,
        id_: uuid::Uuid,
        content_: Option<String>,
        rating_: f64,
    ) -> Result<MovieReview, DatabaseError> {
        use crate::schema::movie_reviews::dsl::*;

        let conn = self.pool.get().await?;
        let user_id = self.user.id;

        Ok(conn
            .interact(move |conn| {
                diesel::update(movie_reviews)
                    .filter(id.eq(id_))
                    .filter(author_user_id.eq(user_id))
                    .set((content.eq(content_), rating.eq(rating_)))
                    .returning(MovieReview::as_returning())
                    .get_result(conn)
            })
            .await??)
    }

    pub async fn delete_review(&self, id_: uuid::Uuid) -> Result<(), DatabaseError> {
        use crate::schema::movie_reviews::dsl::*;

        let conn = self.pool.get().await?;
        let user_id = self.user.id;

        conn.interact(move |conn| {
            diesel::delete(movie_reviews)
                .filter(id.eq(id_))
                .filter(author_user_id.eq(user_id))
                .execute(conn)
        })
        .await??;

        Ok(())
    }
}

#[derive(Clone)]
pub struct TicketResource {
    ticket: Ticket,
    pool: Pool,
}

impl TicketResource {
    fn new(ticket: Ticket, pool: Pool) -> Self {
        Self { ticket, pool }
    }

    pub fn create_jwt(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = JwtClaims {
            dat: JwtType::Ticket(self.ticket.id),
            sub: self.ticket.owner_user_id,
            iat: self.ticket.issued_at.timestamp(),
            exp: self.ticket.expires_at.timestamp(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(jwt_ticket_secret().as_bytes()),
        )
    }

    pub fn verify_jwt(jwt: &str) -> Option<JwtClaims> {
        let data = decode::<JwtClaims>(
            jwt,
            &DecodingKey::from_secret(jwt_ticket_secret().as_bytes()),
            &Validation::new(*JWT_ALGO),
        );

        if let Ok(data) = data {
            Some(data.claims)
        } else {
            None
        }
    }

    async fn set_usage(&self, state: bool) -> Result<(), DatabaseError> {
        use crate::schema::tickets::dsl::*;

        let conn = self.pool.get().await?;
        let ticket = self.ticket.clone();

        conn.interact(move |conn| {
            diesel::update(tickets)
                .filter(id.eq(ticket.id))
                .set(used.eq(state))
                .execute(conn)
        })
        .await??;
        Ok(())
    }

    pub async fn mark_as_used(&self) -> Result<(), DatabaseError> {
        self.set_usage(true).await
    }

    pub async fn mark_as_unused(&self) -> Result<(), DatabaseError> {
        self.set_usage(false).await
    }
}

impl From<UserResource> for User {
    fn from(value: UserResource) -> Self {
        Self { ..value.user }
    }
}

impl From<TicketResource> for Ticket {
    fn from(value: TicketResource) -> Self {
        Self { ..value.ticket }
    }
}
