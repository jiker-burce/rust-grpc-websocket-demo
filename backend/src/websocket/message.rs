use crate::chat::ChatMessage;
use crate::websocket::user_tracker::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    // 实时通信消息
    #[serde(rename = "new_message")]
    NewMessage { message: ChatMessage },
    #[serde(rename = "user_joined")]
    UserJoined {
        user_id: String,
        username: String,
        room_id: String,
    },
    #[serde(rename = "user_left")]
    UserLeft {
        user_id: String,
        username: String,
        room_id: String,
    },
    #[serde(rename = "ping")]
    Ping { timestamp: i64 },
    #[serde(rename = "pong")]
    Pong { timestamp: i64 },

    // WebSocket实时通信消息
    #[serde(rename = "join_room")]
    JoinRoom { room_id: String, user_id: String },
    #[serde(rename = "leave_room")]
    LeaveRoom { room_id: String, user_id: String },
    #[serde(rename = "send_message")]
    SendMessage {
        user_id: String,
        content: String,
        room_id: String,
        message_type: String,
    },
    #[serde(rename = "get_messages")]
    GetMessages { room_id: String, limit: Option<i32> },
    #[serde(rename = "get_online_users")]
    GetOnlineUsers { room_id: String },
    #[serde(rename = "online_users_list")]
    OnlineUsersList {
        room_id: String,
        users: Vec<UserInfo>,
    },

    // gRPC响应消息
    #[serde(rename = "messages_response")]
    MessagesResponse { data: Vec<u8> },
    #[serde(rename = "online_users_response")]
    OnlineUsersResponse { data: Vec<u8> },

    // 通用响应
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
