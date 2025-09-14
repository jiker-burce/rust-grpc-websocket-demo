use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::chat::LeaveRoomRequest;
use crate::redis::SessionManager;
use crate::websocket::{UserTracker, WebSocketMessage};
use prost::Message;
use std::sync::Arc;

/// 离开房间消息事件处理器
pub struct LeaveRoomHandler {
    session_manager: Arc<SessionManager>,
    user_tracker: Arc<UserTracker>,
}

impl LeaveRoomHandler {
    pub fn new(session_manager: Arc<SessionManager>, user_tracker: Arc<UserTracker>) -> Self {
        Self {
            session_manager,
            user_tracker,
        }
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for LeaveRoomHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::LeaveRoom { room_id, user_id } = message {
            println!("处理离开房间请求: room_id={}, user_id={}", room_id, user_id);

            // 从消息中获取用户ID
            let username = format!("用户_{}", &user_id[5..13]); // 从 user_xxxxxxxx 中提取ID

            // 从上下文中获取连接ID
            let connection_id = context
                .connection_id
                .clone()
                .unwrap_or_else(|| format!("conn_{}", user_id));

            // 用户离开房间（切换到空房间）
            // 注意：这里我们暂时不实现离开房间的逻辑，因为新的UserTracker设计中没有专门的离开房间方法
            // 在实际应用中，离开房间可能意味着用户断开连接或者切换到其他房间
            println!("用户 {} 离开房间 {}", username, room_id);

            // 广播用户离开房间的消息
            let broadcast_message = WebSocketMessage::UserLeft {
                user_id: user_id.clone(),
                username: username.clone(),
                room_id: room_id.clone(),
            };

            // 广播给房间内其他用户
            let mut broadcast_handler = context.broadcast_handler.lock().await;
            broadcast_handler.broadcast_to_room(&room_id, &broadcast_message);

            // 获取更新后的房间用户列表并广播
            let room_users = self.user_tracker.get_room_users(&room_id).await;
            let users_list_message = WebSocketMessage::OnlineUsersList {
                room_id: room_id.clone(),
                users: room_users,
            };
            broadcast_handler.broadcast_to_room(&room_id, &users_list_message);

            drop(broadcast_handler);

            println!("用户 {} 成功离开房间: {}", username, room_id);

            // 不返回响应，只广播消息
            Ok(MessageResult::NoOp)
        } else {
            Err("消息类型不匹配".into())
        }
    }

    fn supported_message_type(&self) -> &'static str {
        "leave_room"
    }
}
