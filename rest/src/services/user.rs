use crate::handlers::ErrorType;
use crate::mailer::Mailer;
use crate::util::JWT_ALGO;
use deadpool_diesel::postgres::Pool;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use either::Either::{self, Left};
use jsonwebtoken::Validation;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::response::Response;
use lettre::{Address, Message, SmtpTransport, Transport};
use rayon::prelude::*;
use serde::Serialize;
use std::str::FromStr;
use utoipa::{ToResponse, ToSchema};

use super::DatabaseError;
use crate::model::*;
use crate::password;
use crate::vars::{
    gmail_user, jwt_email_secret, jwt_ticket_secret, jwt_user_secret, server_domain, server_port,
    server_protocol,
};

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
        email_: String,
        username_: String,
    ) -> Result<Option<UserResource>, DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;

        let result = conn
            .interact(|conn| {
                users
                    .limit(1)
                    .filter(is_deleted.eq(false))
                    .filter(email.eq(email_))
                    .or_filter(username.eq(username_))
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
                diesel::update(users.filter(id.eq(id_)).filter(is_deleted.eq(false)))
                    .set(is_deleted.eq(true))
                    .execute(conn)
            })
            .await??;

        Ok(())
    }
}

#[derive(Serialize, ToSchema)]
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
        let Some(jwt_user_secret) = jwt_user_secret() else {
            return Err(DatabaseError::Other("Problem building an email, because of jwt_user_secret var missing".to_string()));
        };

        let token = encode(
            &Header::new(*JWT_ALGO),
            &JwtClaims {
                dat: JwtType::User(self.user.id),
                sub: self.user.id,
                iat: (chrono::Utc::now()).timestamp(),
                exp: (chrono::Utc::now() + chrono::Duration::days(USER_TOKEN_EXPIRY_DAYS))
                    .timestamp(),
            },
            &EncodingKey::from_secret(jwt_user_secret.as_bytes()),
        )?;

        Ok(LoginResponse { token })
    }

    pub fn verify_user_jwt(user_jwt: &str) -> Option<JwtClaims> {
        let Some(jwt_user_secret) = jwt_user_secret() else {
            return None;
        };

        let data = decode::<JwtClaims>(
            user_jwt,
            &DecodingKey::from_secret(jwt_user_secret.as_bytes()),
            &Validation::new(*JWT_ALGO),
        );

        if let Ok(data) = data {
            Some(data.claims)
        } else {
            None
        }
    }

    pub fn create_email_jwt(&self) -> Result<String, DatabaseError> {
        let Some(jwt_email_secret) = jwt_email_secret() else {
            return Err(DatabaseError::Other("Problem building an email, because of jwt_email_secret var missing".to_string()));
        };

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
            &EncodingKey::from_secret(jwt_email_secret.as_ref()),
        )?)
    }

    pub fn verify_email_jwt(email_jwt: &str) -> Result<JwtClaims, DatabaseError> {
        let Some(jwt_email_secret) = jwt_email_secret() else {
            return Err(DatabaseError::Other("Problem building an email, because of jwt_email_secret var missing".to_string()));
        };

        Ok(decode::<JwtClaims>(
            email_jwt,
            &DecodingKey::from_secret(jwt_email_secret.as_bytes()),
            &Validation::new(*JWT_ALGO),
        )?
        .claims)
    }

    pub fn get_email_jwt_url(&self) -> Result<Message, DatabaseError> {
        let Some(from_address) = gmail_user() else {
            return Err(DatabaseError::Other("Problem building an email, because of gmail_user var missing".to_string()));
        };

        let Some(server_domain) = server_domain() else {
            return Err(DatabaseError::Other("Problem building an email, because of server_domain var missing".to_string()));
        };

        let Some(server_port) = server_port() else {
            return Err(DatabaseError::Other("Problem building an email, because of server_port var missing".to_string()));
        };

        let Some(server_protocol) = server_protocol() else {
            return Err(DatabaseError::Other("Problem building an email, because of server_protocol var missing".to_string()));
        };

        let token = self.create_email_jwt()?;
        let from_address = Address::from_str(&from_address)?;
        let to_address = Address::from_str(&self.user.email)?;

        let body = format!(
            "
    <h1>Your verification URL is below</h1>
    <a href=\"{server_protocol}://{server_domain}:{server_port}/api/v1/auth/verify?email_key={token}\">{token}</a>
    <h2>If you weren't expecting this email, you can ignore it.</h2>
    "
        );

        let email = Message::builder()
            .from(Mailbox::new(Some("Nice Movies".to_owned()), from_address))
            .to(Mailbox::new(None, to_address))
            .subject("Verification Cinemaroo")
            .header(ContentType::TEXT_HTML)
            .body(body)?;

        Ok(email)
    }

    pub async fn activate(&mut self) -> Result<(), DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;
        let user = self.user.clone();

        conn.interact(move |conn| {
            diesel::update(users.filter(id.eq(user.id)).filter(is_deleted.eq(false)))
                .set(is_activated.eq(true))
                .execute(conn)
        })
        .await??;

        self.user.is_activated = true;

        Ok(())
    }

    pub async fn update_user(&mut self, new_user: UpdateUser) -> Result<(), DatabaseError> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;
        let user = self.user.clone();

        let result = conn
            .interact(move |conn| {
                diesel::update(users.filter(id.eq(user.id)).filter(is_deleted.eq(false)))
                    .set(new_user)
                    .returning(User::as_returning())
                    .get_result(conn)
            })
            .await??;

        self.user = result;

        Ok(())
    }

    pub async fn update_password(&mut self, new_password: String) -> Result<(), DatabaseError> {
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

                    Ok(
                        diesel::update(users.filter(id.eq(id_)).filter(is_deleted.eq(false)))
                            .set(password_hash.eq(hash))
                            .returning(User::as_returning())
                            .get_result(conn)?,
                    )
                }
                Err(_) => Err(NotFound),
            }
        }

        let res = conn
            .interact(move |conn| insert_password(user.id, new_password.as_bytes(), conn))
            .await??;
        self.user.password_hash = res.password_hash;
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
        new_ticket: FormTicket,
    ) -> Result<TicketResource, DatabaseError> {
        let conn = self.pool.get().await?;

        let ticket = CreateTicket {
            owner_user_id: self.user.id,
            theatre_screening_id: new_ticket.theatre_screening_id,
            ticket_type_id: new_ticket.ticket_type_id,
            issuer_user_id: new_ticket.issuer_user_id,
            seat_row: new_ticket.seat_row,
            seat_column: new_ticket.seat_column,
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

    pub async fn get_reviews(&self) -> Result<Vec<ExtendedMovieReview>, DatabaseError> {
        use crate::schema::*;

        let conn = self.pool.get().await?;
        let cloned_user = self.user.clone();

        Ok(conn
            .interact(move |conn| {
                MovieReview::belonging_to(&cloned_user)
                    .inner_join(movies::table)
                    .select((PartialMovie::as_select(), PartialMovieReview::as_select()))
                    .load(conn)
            })
            .await??)
    }

    pub async fn create_review(
        &self,
        review: FormMovieReview,
    ) -> Result<MovieReview, DatabaseError> {
        let conn = self.pool.get().await?;

        let review = CreateMovieReview {
            author_user_id: self.user.id,
            content: review.content,
            movie_id: review.movie_id,
            rating: review.rating,
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
                diesel::update(
                    movie_reviews
                        .filter(id.eq(id_))
                        .filter(author_user_id.eq(user_id)),
                )
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
    pub fn new(ticket: Ticket, pool: Pool) -> Self {
        Self { ticket, pool }
    }

    pub fn create_jwt(&self) -> Result<String, Either<(), jsonwebtoken::errors::Error>> {
        let Some(jwt_ticket_secret) = jwt_ticket_secret() else {
            return Err(Either::Left(()));
        };

        let claims = JwtClaims {
            dat: JwtType::Ticket(self.ticket.id),
            sub: self.ticket.owner_user_id,
            iat: self.ticket.issued_at.timestamp(),
            exp: self.ticket.expires_at.timestamp(),
        };

        match jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(jwt_ticket_secret.as_bytes()),
        ) {
            Ok(v) => Ok(v),
            Err(e) => Err(Either::Right(e)),
        }
    }

    pub fn verify_jwt(jwt: &str) -> Result<uuid::Uuid, DatabaseError> {
        let Some(jwt_ticket_secret) = jwt_ticket_secret() else {
            return Err(DatabaseError::Other("Problem building an email, because of jwt_ticket_secret var missing".to_string()));
        };

        let data = decode::<JwtClaims>(
            jwt,
            &DecodingKey::from_secret(jwt_ticket_secret.as_bytes()),
            &Validation::new(*JWT_ALGO),
        )?;

        match data.claims.dat {
            JwtType::Ticket(id) => Ok(id),
            _ => Err(DatabaseError::Other("Invalid JWT token".to_string())),
        }
    }

    async fn set_usage(&mut self, state: bool) -> Result<(), DatabaseError> {
        use crate::schema::tickets::dsl::*;

        let conn = self.pool.get().await?;
        let ticket = self.ticket.clone();

        conn.interact(move |conn| {
            diesel::update(tickets.filter(id.eq(ticket.id)))
                .set(used.eq(state))
                .execute(conn)
        })
        .await??;

        self.ticket.used = state;

        Ok(())
    }

    pub async fn mark_as_used(&mut self) -> Result<(), DatabaseError> {
        self.set_usage(true).await
    }

    pub async fn mark_as_unused(&mut self) -> Result<(), DatabaseError> {
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
