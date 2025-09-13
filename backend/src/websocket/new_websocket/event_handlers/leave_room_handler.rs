use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::redis::SessionManager;
use crate::websocket::WebSocketMessage;
use std::sync::Arc;

/// 离开房间消息事件处理器
pub struct LeaveRoomHandler {
    session_manager: Arc<SessionManager>,
}

impl LeaveRoomHandler {
    pub fn new(session_manager: Arc<SessionManager>) -> Self {
        Self { session_manager }
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for LeaveRoomHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::LeaveRoom {
            room_id,
            user_id: uid,
        } = message
        {
            println!("用户 {} 离开房间: {}", uid, room_id);

            // 从房间的Redis列表中移除用户
            if let Err(e) = self
                .session_manager
                .remove_user_from_room(&uid, &room_id)
                .await
            {
                eprintln!("从房间移除用户失败: {}", e);
            }

            // 广播用户离开房间的消息
            let user_offline_msg = WebSocketMessage::UserOffline {
                user_id: uid.clone(),
            };

            let mut broadcast_handler = context.broadcast_handler.lock().await;
            if let Some(room_tx) = broadcast_handler.get_room_channel(&room_id) {
                if let Err(e) = room_tx.send(user_offline_msg) {
                    eprintln!("广播用户离开房间消息失败: {}", e);
                }
            }

            return Ok(MessageResult::ClearCurrentRoom);
        }
        Ok(MessageResult::NoOp)
    }

    fn supported_message_type(&self) -> &'static str {
        "leave_room"
    }
}
