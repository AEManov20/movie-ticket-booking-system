use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::model::{Role, TheatreRole, UserTheatreRole};

use super::DatabaseError;

/// this service represents the
/// 'theatre_roles' table
#[derive(Clone)]
pub struct RoleService {
    pool: Pool,
}

impl RoleService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// fetches a `Role` by id
    pub async fn get_role_by_id(&self, rid: uuid::Uuid) -> Result<Option<Role>, DatabaseError> {
        use crate::schema::theatre_roles::dsl::*;

        let t_role = self
            .pool
            .get()
            .await?
            .interact(move |conn| {
                theatre_roles
                    .filter(id.eq(rid))
                    .limit(1)
                    .load::<TheatreRole>(conn)
            })
            .await??
            .first()
            .cloned();

        if let Some(t_role) = t_role {
            Ok(Role::try_from_str(&t_role.name))
        } else {
            Ok(None)
        }
    }

    /// fetches a `TheatreRole` by name
    pub async fn get_role_by_name(&self, nm: String) -> Result<Option<TheatreRole>, DatabaseError> {
        use crate::schema::theatre_roles::dsl::*;

        Ok(self
            .pool
            .get()
            .await?
            .interact(move |conn| theatre_roles.filter(name.eq(nm)).limit(1).load(conn))
            .await??
            .first()
            .cloned())
    }

    pub async fn get_all_roles(&self) -> Result<Vec<TheatreRole>, DatabaseError> {
        use crate::schema::theatre_roles::dsl::*;

        Ok(self
            .pool
            .get()
            .await?
            .interact(move |conn| theatre_roles.load(conn))
            .await??)
    }
}
