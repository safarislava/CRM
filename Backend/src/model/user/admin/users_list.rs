use crate::common::BoxError;
use crate::model::project::contract::json::Json;
use crate::model::user::role::Role;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct UsersList {
    pool: Arc<PgPool>,
}

impl UsersList {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    email: String,
    notifications_enabled: bool,
    created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct RoleRow {
    user_id: Uuid,
    role: Role,
}

#[derive(Serialize)]
struct UserItem {
    id: Uuid,
    username: String,
    email: String,
    notifications_enabled: bool,
    created_at: DateTime<Utc>,
    roles: Vec<Role>,
}

#[async_trait::async_trait]
impl Json for UsersList {
    async fn json(&self) -> Result<serde_json::Value, BoxError> {
        let users = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, email, notifications_enabled, created_at FROM users ORDER BY created_at DESC",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        let role_rows = sqlx::query_as::<_, RoleRow>("SELECT user_id, role FROM user_roles")
            .fetch_all(self.pool.as_ref())
            .await?;
        let mut user_roles: HashMap<Uuid, Vec<Role>> = HashMap::new();
        for r in role_rows {
            user_roles.entry(r.user_id).or_default().push(r.role);
        }
        let items: Vec<UserItem> = users
            .into_iter()
            .map(|u| {
                let roles = user_roles.remove(&u.id).unwrap_or_default();
                UserItem {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                    notifications_enabled: u.notifications_enabled,
                    created_at: u.created_at,
                    roles,
                }
            })
            .collect();
        Ok(serde_json::to_value(items)?)
    }
}
