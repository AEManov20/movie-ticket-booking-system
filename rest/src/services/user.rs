use chrono::Utc;
use deadpool_diesel::postgres::{Manager, Pool};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use jsonwebtoken::Header;

use crate::model::*;
use crate::password;

pub struct UserService {
    pool: Pool,
}

impl UserService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user: FormUser,
    ) -> Result<Option<UserResource>, Box<dyn std::error::Error>> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;

        let hash = password::hash(user.password.as_bytes())?;

        if !password::verify(user.password.as_bytes(), &hash) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Hash failed verification",
            )));
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
                    .load(conn)
            })
            .await??
            .first()
            .cloned();

        match result {
            Some(user) => Ok(Some(UserResource::new(user, self.pool.clone()))),
            None => Ok(None),
        }
    }

    pub async fn get_by_username(
        &self,
        username_: String,
    ) -> Result<Option<UserResource>, Box<dyn std::error::Error>> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;

        let result = conn
            .interact(|conn| {
                users
                    .filter(username.eq(username_))
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

    pub async fn get_by_id(
        &self,
        id_: i32,
    ) -> Result<Option<UserResource>, Box<dyn std::error::Error>> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().await?;

        let result = conn
            .interact(move |conn| users.filter(id.eq(id_)).limit(1).load::<User>(conn))
            .await??
            .first()
            .cloned();

        match result {
            Some(user) => Ok(Some(UserResource::new(user, self.pool.clone()))),
            None => Ok(None),
        }
    }

    pub async fn delete(&self, id_: i32) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::users::dsl::*;

        self.pool
            .get()
            .await?
            .interact(move |conn| {
                diesel::update(users)
                    .filter(id.eq(id_))
                    .set(is_deleted.eq(true))
                    .execute(conn)
            })
            .await??;

        Ok(())
    }
}

pub struct UserResource {
    user: User,
    pool: Pool,
}

impl UserResource {
    fn new(user: User, pool: Pool) -> Self {
        Self { user, pool }
    }

    pub async fn get_theatre_permissions(
        &mut self,
    ) -> Result<Vec<TheatrePermission>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let user = self.user.clone();

        Ok(conn
            .interact(move |conn| TheatrePermission::belonging_to(&user).load(conn))
            .await??)
    }

    pub async fn create_theatre_permission(
        &self,
        theatre_id: i32,
        can_check_tickets: bool,
        can_manage_movies: bool,
        can_manage_tickets: bool,
        can_manage_users: bool,
        is_theatre_owner: bool,
    ) -> Result<Option<TheatrePermission>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let permission = FormTheatrePermission {
            user_id: self.user.id,
            can_check_tickets,
            can_manage_movies,
            can_manage_tickets,
            can_manage_users,
            is_theatre_owner,
            theatre_id,
        };

        Ok(conn
            .interact(move |conn| {
                diesel::insert_into(TheatrePermission::table())
                    .values(permission)
                    .load::<TheatrePermission>(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn update_theatre_permission(
        &self,
        id_: i32,
        can_check_tickets: bool,
        can_manage_movies: bool,
        can_manage_tickets: bool,
        can_manage_users: bool,
        is_theatre_owner: bool,
    ) -> Result<Option<TheatrePermission>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let update = UpdateTheatrePermission {
            can_check_tickets,
            can_manage_movies,
            can_manage_tickets,
            can_manage_users,
            is_theatre_owner,
        };

        Ok(conn
            .interact(move |conn| {
                diesel::update(TheatrePermission::table())
                    .filter(crate::schema::theatre_permissions::dsl::id.eq(id_))
                    .set(update)
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn delete_theatre_permission(
        &mut self,
        id_: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::theatre_permissions::dsl::*;

        let conn = self.pool.get().await?;

        conn.interact(move |conn| {
            diesel::delete(theatre_permissions)
                .filter(id.eq(id_))
                .execute(conn)
        })
        .await??;
        Ok(())
    }

    pub async fn update_user(
        &mut self,
        new_user: FormUser,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                    .load(conn)
            })
            .await??
            .first()
            .cloned();

        let Some(result) = result else {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Database returned nothing").into())
        };

        self.user = result;

        Ok(())
    }

    pub async fn update_password(
        &mut self,
        old_password: String,
        new_password: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;
        let user = self.user.clone();

        fn insert_password(
            id_: i32,
            pass: &[u8],
            conn: &mut PgConnection,
        ) -> QueryResult<Option<User>> {
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
                        .load(conn)?
                        .first()
                        .cloned())
                }
                Err(_) => Err(NotFound),
            }
        }

        let Some(ref hash) = self.user.password_hash else {
            match conn.interact(move |conn| insert_password(user.id, new_password.as_bytes(), conn)).await?? {
                Some(v) => self.user.password_hash = v.password_hash,
                None => return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Password could not be updated",
                )))
            };
            return Ok(())
        };

        if !password::verify(old_password.as_bytes(), hash) {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Invalid password",
            )))
        } else {
            conn.interact(move |conn| insert_password(user.id, new_password.as_bytes(), conn))
                .await??;
            Ok(())
        }
    }

    pub async fn get_tickets(&mut self) -> Result<Vec<TicketResource>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;
        let cloned_user = self.user.clone();

        Ok(conn
            .interact(move |conn| Ticket::belonging_to(&cloned_user).load(conn))
            .await??
            .iter()
            .map(|e: &Ticket| TicketResource::new(e.clone(), self.pool.clone()))
            .collect::<Vec<_>>())
    }

    pub async fn create_ticket(
        &mut self,
        issuer_user_id: Option<i32>,
        expires_at: chrono::NaiveDateTime,
        theatre_movie_id: i32,
        ticket_type_id: i32,
        seat_row: i32,
        seat_column: i32,
    ) -> Result<Option<TicketResource>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let ticket = FormTicket {
            owner_user_id: self.user.id,
            theatre_movie_id,
            ticket_type_id,
            issuer_user_id,
            expires_at,
            seat_row,
            seat_column,
        };

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(Ticket::table())
                    .values(ticket)
                    .load(conn)
            })
            .await??
            .first()
            .cloned();

        match result {
            Some(v) => Ok(Some(TicketResource::new(v, self.pool.clone()))),
            None => Ok(None),
        }
    }

    pub async fn delete_ticket(&mut self, id_: i32) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::tickets::dsl::*;

        let conn = self.pool.get().await?;

        conn.interact(move |conn| diesel::delete(tickets).filter(id.eq(id_)).execute(conn))
            .await??;

        Ok(())
    }

    pub async fn get_reviews(&mut self) -> Result<Vec<MovieReview>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;
        let cloned_user = self.user.clone();

        Ok(conn
            .interact(move |conn| MovieReview::belonging_to(&cloned_user).load(conn))
            .await??)
    }

    pub async fn create_review(
        &mut self,
        content: Option<String>,
        rating: f64,
        movie_id: i32,
    ) -> Result<Option<MovieReview>, Box<dyn std::error::Error>> {
        let conn = self.pool.get().await?;

        let review = FormMovieReview {
            author_user_id: self.user.id,
            content,
            rating,
            movie_id,
        };

        Ok(conn
            .interact(move |conn| {
                diesel::insert_into(MovieReview::table())
                    .values(&review)
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn update_review(
        &mut self,
        id_: i32,
        content_: Option<String>,
        rating_: f64,
    ) -> Result<Option<MovieReview>, Box<dyn std::error::Error>> {
        use crate::schema::movie_reviews::dsl::*;

        let conn = self.pool.get().await?;
        let user_id = self.user.id;

        Ok(conn
            .interact(move |conn| {
                diesel::update(movie_reviews)
                    .filter(id.eq(id_))
                    .filter(author_user_id.eq(user_id))
                    .set((content.eq(content_), rating.eq(rating_)))
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
    }

    pub async fn delete_review(&mut self, id_: i32) -> Result<(), Box<dyn std::error::Error>> {
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
            iss: self.ticket.issuer_user_id,
            iat: Utc::now().timestamp(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(crate::vars::jwt_ticket_secret().as_bytes()),
        )
    }

    async fn set_usage(&self, state: bool) -> Result<(), Box<dyn std::error::Error>> {
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

    pub async fn mark_as_used(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_usage(true).await
    }

    pub async fn mark_as_unused(&self) -> Result<(), Box<dyn std::error::Error>> {
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
