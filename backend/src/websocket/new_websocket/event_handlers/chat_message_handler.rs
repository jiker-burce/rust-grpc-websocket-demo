use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::database::{MessageRepository, UserRepository};
use crate::models::{Message, MessageType};
use crate::websocket::WebSocketMessage;
use std::sync::Arc;

/// 聊天消息事件处理器
pub struct ChatMessageHandler {
    user_repo: Arc<UserRepository>,
    message_repo: Arc<MessageRepository>,
}

impl ChatMessageHandler {
    pub fn new(user_repo: Arc<UserRepository>, message_repo: Arc<MessageRepository>) -> Self {
        Self {
            user_repo,
            message_repo,
        }
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for ChatMessageHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::ChatMessage {
            room_id,
            user_id: uid,
            content,
            message_type,
            username,
        } = message
        {
            println!(
                "收到聊天消息: room_id={}, user_id={}, content={}",
                room_id, uid, content
            );

            // 验证用户
            if let Some(_u) = self.user_repo.find_by_id(&uid).await? {
                println!("找到用户: {}", username);

                let msg_type = match message_type.as_str() {
                    "image" => MessageType::Image,
                    "file" => MessageType::File,
                    _ => MessageType::Text,
                };

                let message = Message::new(
                    uid.clone(),
                    username.clone(),
                    content.clone(),
                    room_id.clone(),
                    msg_type,
                );

                // 保存到数据库
                match self.message_repo.create(message).await {
                    Ok(_) => println!("消息已保存到数据库"),
                    Err(e) => {
                        println!("保存消息到数据库失败: {}", e);
                        return Ok(MessageResult::NoOp);
                    }
                }

                // 广播消息到房间
                let broadcast_msg = WebSocketMessage::ChatMessage {
                    room_id: room_id.clone(),
                    user_id: uid,
                    username,
                    content,
                    message_type,
                };
                println!("准备广播消息到房间: {}", room_id);

                // 获取对应房间的广播通道
                let mut broadcast_handler = context.broadcast_handler.lock().await;
                let room_tx = broadcast_handler.get_or_create_room_channel(&room_id);
                let _ = room_tx.send(broadcast_msg);
                println!("消息已广播到房间: {}", room_id);
            } else {
                println!("用户不存在: {}", uid);
            }
        }
        Ok(MessageResult::NoOp)
    }

    fn supported_message_type(&self) -> &'static str {
        "chat_message"
    }
}
