use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub avatar: Option<String>,
    pub is_online: Option<i8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub avatar: Option<String>,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            email,
            password_hash,
            avatar: None,
            is_online: Some(0),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn to_public(&self) -> PublicUser {
        PublicUser {
            id: self.id.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
            avatar: self.avatar.clone(),
            is_online: self.is_online,
            created_at: self.created_at.timestamp(),
            updated_at: self.updated_at.timestamp(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub is_online: Option<i8>,
    pub created_at: i64,
    pub updated_at: i64,
}
