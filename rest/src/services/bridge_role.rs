use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::model::{UserTheatreRole, Role};

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

    pub async fn register_role(
        &self,
        role: UserTheatreRole,
    ) -> Result<Option<UserTheatreRole>, DatabaseError> {
        use crate::schema::users_theatre_roles::dsl::*;
        Ok(self
            .pool
            .get()
            .await?
            .interact(move |conn| {
                diesel::insert_into(users_theatre_roles)
                    .values(role)
                    .load(conn)
            })
            .await??
            .first()
            .cloned())
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
}
