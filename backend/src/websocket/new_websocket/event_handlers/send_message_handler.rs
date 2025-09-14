use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::chat::ChatMessage;
use crate::database::MessageRepository;
use crate::websocket::{BroadcastHandler, UserTracker, WebSocketMessage};
use std::sync::Arc;
use tokio::sync::Mutex;

/// WebSocket发送消息处理器 - 专注实时通信
/// 职责：接收消息并通过WebSocket广播给其他用户
pub struct SendMessageHandler {
    // 用于存储房间连接，用于广播消息
    room_connections: Arc<Mutex<std::collections::HashMap<String, Vec<String>>>>,
    // 消息仓库，用于存储消息到数据库
    message_repo: Arc<MessageRepository>,
    // 用户跟踪器，用于获取在线用户信息
    user_tracker: Arc<UserTracker>,
    // 广播处理器，用于广播消息
    broadcast_handler: Arc<Mutex<BroadcastHandler>>,
}

impl SendMessageHandler {
    pub fn new(
        message_repo: Arc<MessageRepository>,
        user_tracker: Arc<UserTracker>,
        broadcast_handler: Arc<Mutex<BroadcastHandler>>,
    ) -> Self {
        Self {
            room_connections: Arc::new(Mutex::new(std::collections::HashMap::new())),
            message_repo,
            user_tracker,
            broadcast_handler,
        }
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for SendMessageHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::SendMessage {
            user_id,
            content,
            room_id,
            message_type,
        } = message
        {
            println!(
                "WebSocket处理发送消息: room_id={}, user_id={}, content={}, message_type={}",
                room_id, user_id, content, message_type
            );

            // 将字符串类型的message_type转换为i32
            let message_type_i32 = match message_type.as_str() {
                "text" => 0,   // MessageType::Text
                "image" => 1,  // MessageType::Image
                "file" => 2,   // MessageType::File
                "system" => 3, // MessageType::System
                _ => 0,        // 默认为文本类型
            };

            // 从UserTracker获取用户信息（内存级别，高效）
            let connection_id = context
                .connection_id
                .clone()
                .ok_or_else(|| "连接ID不存在")?;
            let user_info = self
                .user_tracker
                .get_user_by_connection(&connection_id)
                .await
                .ok_or_else(|| format!("用户不在线: {}", user_id))?;

            // 创建聊天消息用于广播
            let chat_message = ChatMessage {
                id: format!("msg_{}", chrono::Utc::now().timestamp_millis()),
                user_id: user_id.clone(),
                username: user_info.username.clone(),
                content,
                room_id: room_id.clone(),
                message_type: message_type_i32,
                timestamp: chrono::Utc::now().timestamp_millis(),
            };

            // 存储消息到数据库
            self.store_message_to_database(&chat_message).await?;

            // 通过WebSocket广播消息给房间内所有用户（排除发送者）
            self.broadcast_message_to_room(&room_id, &chat_message, &user_id)
                .await?;

            // 不返回响应，只广播消息
            Ok(MessageResult::NoOp)
        } else {
            Err("消息类型不匹配".into())
        }
    }

    fn supported_message_type(&self) -> &'static str {
        "send_message"
    }
}

impl SendMessageHandler {
    /// 存储消息到数据库
    async fn store_message_to_database(
        &self,
        message: &ChatMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 将ChatMessage转换为数据库Message格式
        let db_message = crate::models::Message {
            id: message.id.clone(),
            user_id: message.user_id.clone(),
            username: message.username.clone(),
            content: message.content.clone(),
            room_id: message.room_id.clone(),
            message_type: match message.message_type {
                0 => crate::models::MessageType::Text,
                1 => crate::models::MessageType::Image,
                2 => crate::models::MessageType::File,
                3 => crate::models::MessageType::System,
                _ => crate::models::MessageType::Text,
            },
            created_at: chrono::DateTime::from_timestamp_millis(message.timestamp)
                .unwrap_or_else(|| chrono::Utc::now()),
        };

        // 存储到数据库
        self.message_repo.create(db_message).await?;
        println!("成功存储消息到数据库: {}", message.id);

        Ok(())
    }

    /// 向房间内所有用户广播消息（排除发送者）
    async fn broadcast_message_to_room(
        &self,
        room_id: &str,
        message: &ChatMessage,
        sender_user_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 创建广播消息
        let broadcast_message = WebSocketMessage::NewMessage {
            message: message.clone(),
        };

        // 使用BroadcastHandler广播消息
        let mut broadcast_handler = self.broadcast_handler.lock().await;
        broadcast_handler.broadcast_to_room(room_id, &broadcast_message);

        println!(
            "成功广播消息到房间 {} (排除发送者 {}): {:?}",
            room_id, sender_user_id, message
        );

        Ok(())
    }
}
