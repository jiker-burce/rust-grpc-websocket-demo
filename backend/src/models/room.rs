use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<i8>,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoom {
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
}

impl Room {
    pub fn new(
        name: String,
        description: Option<String>,
        is_public: bool,
        created_by: String,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            is_public: Some(is_public as i8),
            created_by,
            created_at: now,
            updated_at: now,
        }
    }
}
