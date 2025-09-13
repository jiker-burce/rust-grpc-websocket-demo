use crate::database::{MessageRepository, UserRepository};
use crate::models::{Message, MessageType};
use crate::redis::SessionManager;
use crate::websocket::{BroadcastHandler, WebSocketMessage};
use std::sync::Arc;
use tokio::sync::Mutex;

/// 消息处理器，负责处理不同类型的WebSocket消息
pub struct MessageHandlers {
    user_repo: Arc<UserRepository>,
    message_repo: Arc<MessageRepository>,
    session_manager: Arc<SessionManager>,
    broadcast_handler: Arc<Mutex<BroadcastHandler>>,
}

impl MessageHandlers {
    pub fn new(
        user_repo: Arc<UserRepository>,
        message_repo: Arc<MessageRepository>,
        session_manager: Arc<SessionManager>,
        broadcast_handler: Arc<Mutex<BroadcastHandler>>,
    ) -> Self {
        Self {
            user_repo,
            message_repo,
            session_manager,
            broadcast_handler,
        }
    }

    /// 处理聊天消息
    pub async fn handle_chat_message(
        &self,
        msg: WebSocketMessage,
        user_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::ChatMessage {
            room_id,
            user_id: uid,
            content,
            message_type,
            username,
        } = msg
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
                        return Ok(());
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
                let mut broadcast_handler = self.broadcast_handler.lock().await;
                let room_tx = broadcast_handler.get_or_create_room_channel(&room_id);
                let _ = room_tx.send(broadcast_msg);
                println!("消息已广播到房间: {}", room_id);
            } else {
                println!("用户不存在: {}", uid);
            }
        }
        Ok(())
    }

    /// 处理加入房间消息
    pub async fn handle_join_room(
        &self,
        msg: WebSocketMessage,
        user_id: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::JoinRoom {
            room_id,
            user_id: uid,
        } = msg
        {
            println!("用户 {} 加入房间: {}", uid, room_id);

            // 验证用户
            if self.user_repo.find_by_id(&uid).await?.is_some() {
                // 广播用户加入房间的消息（暂时不实现，因为WebSocketMessage中没有UserJoined类型）
                // TODO: 如果需要用户加入/离开通知，需要在WebSocketMessage中添加相应类型

                return Ok(Some(room_id));
            }
        }
        Ok(None)
    }

    /// 处理离开房间消息
    pub async fn handle_leave_room(
        &self,
        msg: WebSocketMessage,
        user_id: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::LeaveRoom {
            room_id,
            user_id: uid,
        } = msg
        {
            println!("用户 {} 离开房间: {}", uid, room_id);

            // 广播用户离开房间的消息（暂时不实现，因为WebSocketMessage中没有UserLeft类型）
            // TODO: 如果需要用户加入/离开通知，需要在WebSocketMessage中添加相应类型

            return Ok(Some(room_id));
        }
        Ok(None)
    }

    /// 处理错误消息
    pub async fn handle_error(
        &self,
        msg: WebSocketMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::Error { message } = msg {
            println!("收到错误消息: {}", message);
        }
        Ok(())
    }
}
