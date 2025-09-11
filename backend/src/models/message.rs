use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
    pub room_id: String,
    pub message_type: MessageType,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_type", rename_all = "lowercase")]
pub enum MessageType {
    Text,
    Image,
    File,
    System,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Text => write!(f, "text"),
            MessageType::Image => write!(f, "image"),
            MessageType::File => write!(f, "file"),
            MessageType::System => write!(f, "system"),
        }
    }
}

impl Message {
    pub fn new(
        user_id: String,
        username: String,
        content: String,
        room_id: String,
        message_type: MessageType,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            username,
            content,
            room_id,
            message_type,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn to_grpc(&self) -> crate::chat::ChatMessage {
        crate::chat::ChatMessage {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            username: self.username.clone(),
            content: self.content.clone(),
            room_id: self.room_id.clone(),
            message_type: self.message_type.clone() as i32,
            timestamp: self.created_at.timestamp(),
        }
    }
}

impl From<i32> for MessageType {
    fn from(value: i32) -> Self {
        match value {
            0 => MessageType::Text,
            1 => MessageType::Image,
            2 => MessageType::File,
            3 => MessageType::System,
            _ => MessageType::Text,
        }
    }
}

impl From<Option<String>> for MessageType {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => match s.as_str() {
                "text" => MessageType::Text,
                "image" => MessageType::Image,
                "file" => MessageType::File,
                "system" => MessageType::System,
                _ => MessageType::Text,
            },
            None => MessageType::Text,
        }
    }
}
