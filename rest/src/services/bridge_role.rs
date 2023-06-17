use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::model::UserTheatreRole;

use super::DatabaseError;

/// This service represents the bridge table
/// which goes by the name 'users_theatre_roles'
#[derive(Clone)]
pub struct BridgeRoleService {
    pool: Pool,
}

impl BridgeRoleService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn register_roles(
        &self,
        roles: Vec<UserTheatreRole>,
    ) -> Result<Vec<UserTheatreRole>, DatabaseError> {
        use crate::schema::users_theatre_roles::dsl::*;
        Ok(self
            .pool
            .get()
            .await?
            .interact(move |conn| {
                diesel::insert_into(users_theatre_roles)
                    .values(roles)
                    .returning(UserTheatreRole::as_returning())
                    .get_results(conn)
            })
            .await??)
    }

    pub async fn unregister_roles(
        &self,
        uid: Option<uuid::Uuid>,
        tid: Option<uuid::Uuid>,
        rid: Option<uuid::Uuid>,
    ) -> Result<(), DatabaseError> {
        use crate::schema::users_theatre_roles::dsl::*;

        let conn = self.pool.get().await?;
        let mut query = diesel::delete(users_theatre_roles).into_boxed();

        if let Some(uid) = uid {
            query = query.filter(user_id.eq(uid));
        }

        if let Some(tid) = tid {
            query = query.filter(theatre_id.eq(tid));
        }

        if let Some(rid) = rid {
            query = query.filter(role_id.eq(rid));
        }

        conn.interact(move |conn| query.execute(conn)).await??;

        Ok(())
    }

    pub async fn unregister_roles_batch(
        &self,
        roles: Vec<UserTheatreRole>,
    ) -> Result<(), DatabaseError> {
        use crate::schema::users_theatre_roles::dsl::*;

        let conn = self.pool.get().await?;
        let query = roles.iter().fold(
            diesel::delete(users_theatre_roles).into_boxed(),
            |acc, el| {
                acc.or_filter((
                    role_id.eq(el.role_id),
                    user_id.eq(el.user_id),
                    theatre_id.eq(el.theatre_id),
                ))
            },
        );

        conn.interact(move |conn| query.execute(conn)).await??;
        Ok(())
    }

    pub async fn get_roles(
        &self,
        uid: Option<uuid::Uuid>,
        tid: Option<uuid::Uuid>,
        rid: Option<uuid::Uuid>,
    ) -> Result<Vec<UserTheatreRole>, DatabaseError> {
        use crate::schema::users_theatre_roles::dsl::*;

        let conn = self.pool.get().await?;
        let mut query = users_theatre_roles.into_boxed();

        if let Some(uid) = uid {
            query = query.filter(user_id.eq(uid));
        }

        if let Some(tid) = tid {
            query = query.filter(theatre_id.eq(tid));
        }

        if let Some(rid) = rid {
            query = query.filter(role_id.eq(rid));
        }

        Ok(conn.interact(move |conn| query.load(conn)).await??)
    }

    pub async fn role_exists(&self, role: UserTheatreRole) -> Result<bool, DatabaseError> {
        use crate::schema::users_theatre_roles::dsl::*;

        let conn = self.pool.get().await?;
        Ok(conn
            .interact(move |conn| {
                users_theatre_roles
                    .find((role.user_id, role.role_id, role.theatre_id))
                    .load::<UserTheatreRole>(conn)
            })
            .await??
            .first()
            .is_some())
    }
}

#[macro_export]
macro_rules! check_role {
    ($role:expr, $uid:expr, $tid:expr, $brs:expr, $rs:expr) => {
        if $brs
            .get_roles(
                Some($uid),
                Some($tid),
                $rs.get_role_by_name($role).await?.map(|e| e.id),
            )
            .await?
            .first()
            .is_some()
        {
            true
        } else { false }
    };
}

#[macro_export]
macro_rules! check_roles_or {
    ([$($role:expr),*], $uid:expr, $tid:expr, $brs:expr, $rs:expr) => {
        let matches = vec![$(
            $crate::check_role!($role, $uid, $tid, $brs, $rs)
        ),*];

        if !matches.iter().fold(false, |mut acc, x| { acc = acc || *x; acc }) {
            return Err(ErrorType::InsufficientPermission)
        }
    }
}

#[macro_export]
macro_rules! check_roles_and {
    ([$($role:expr),*], $uid:expr, $tid:expr, $brs:expr, $rs:expr) => {
        let matches = vec![$(
            $crate::check_role!($role, $uid, $tid, $brs, $rs)
        ),*];

        if !matches.iter().fold(false, |mut acc, x| { acc = acc && *x; acc }) {
            return Err(ErrorType::InsufficientPermission)
        }
    }
}
