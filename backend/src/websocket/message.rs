use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    #[serde(rename = "join_room")]
    JoinRoom { room_id: String, user_id: String },
    #[serde(rename = "leave_room")]
    LeaveRoom { room_id: String, user_id: String },
    #[serde(rename = "chat_message")]
    ChatMessage {
        room_id: String,
        user_id: String,
        username: String,
        content: String,
        message_type: String,
    },
    #[serde(rename = "user_online")]
    UserOnline { user_id: String, username: String },
    #[serde(rename = "user_offline")]
    UserOffline { user_id: String },
    #[serde(rename = "error")]
    Error { message: String },
    #[serde(rename = "success")]
    Success { message: String },
}

impl WebSocketMessage {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }
}
