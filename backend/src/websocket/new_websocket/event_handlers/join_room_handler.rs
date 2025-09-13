use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::database::UserRepository;
use crate::redis::SessionManager;
use crate::websocket::WebSocketMessage;
use std::sync::Arc;

/// 加入房间消息事件处理器
pub struct JoinRoomHandler {
    user_repo: Arc<UserRepository>,
    session_manager: Arc<SessionManager>,
}

impl JoinRoomHandler {
    pub fn new(user_repo: Arc<UserRepository>, session_manager: Arc<SessionManager>) -> Self {
        Self {
            user_repo,
            session_manager,
        }
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for JoinRoomHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::JoinRoom {
            room_id,
            user_id: uid,
        } = message
        {
            println!("用户 {} 加入房间: {}", uid, room_id);

            // 验证用户
            if let Some(user) = self.user_repo.find_by_id(&uid).await? {
                // 将用户添加到房间的Redis列表中
                if let Err(e) = self.session_manager.add_user_to_room(&uid, &room_id).await {
                    eprintln!("添加用户到房间失败: {}", e);
                }

                // 广播用户加入房间的消息
                let user_online_msg = WebSocketMessage::UserOnline {
                    user_id: uid.clone(),
                    username: user.username.clone(),
                };

                let mut broadcast_handler = context.broadcast_handler.lock().await;
                let room_tx = broadcast_handler.get_or_create_room_channel(&room_id);
                let receiver = room_tx.subscribe();

                // 广播给房间内的其他用户
                if let Err(e) = room_tx.send(user_online_msg) {
                    eprintln!("广播用户加入房间消息失败: {}", e);
                }

                return Ok(MessageResult::SetRoomReceiver(receiver));
            }
        }
        Ok(MessageResult::NoOp)
    }

    fn supported_message_type(&self) -> &'static str {
        "join_room"
    }
}
