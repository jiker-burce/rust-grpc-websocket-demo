use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::chat::{GetOnlineUsersRequest, GetOnlineUsersResponse};
use crate::websocket::{UserTracker, WebSocketMessage};
use prost::Message;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 处理获取在线用户的protobuf请求
pub struct GetOnlineUsersHandler {
    user_tracker: Arc<UserTracker>,
}

impl GetOnlineUsersHandler {
    pub fn new(user_tracker: Arc<UserTracker>) -> Self {
        Self { user_tracker }
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for GetOnlineUsersHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::GetOnlineUsers { room_id } = message {
            println!("处理获取在线用户请求: room_id={}", room_id);

            // 获取房间用户列表
            let room_users = self.user_tracker.get_room_users(&room_id).await;
            println!("房间 {} 有 {} 个在线用户", room_id, room_users.len());

            // 广播用户列表给房间内所有用户
            let mut broadcast_handler = context.broadcast_handler.lock().await;
            let users_list_message = WebSocketMessage::OnlineUsersList {
                room_id: room_id.clone(),
                users: room_users,
            };
            broadcast_handler.broadcast_to_room(&room_id, &users_list_message);
            drop(broadcast_handler);

            Ok(MessageResult::NoOp)
        } else {
            Err("消息类型不匹配".into())
        }
    }

    fn supported_message_type(&self) -> &'static str {
        "get_online_users"
    }
}
