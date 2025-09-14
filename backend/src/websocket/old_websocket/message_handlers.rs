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
        if let WebSocketMessage::SendMessage { .. } = msg {
            println!("收到发送消息请求，用户ID: {}", user_id);

            // 验证用户
            if let Some(_u) = self.user_repo.find_by_id(user_id).await? {
                println!("找到用户: {}", user_id);

                // 简化处理，暂时跳过消息创建
                println!("处理发送消息请求完成");

                // 简化处理，暂时跳过数据库操作和广播
                println!("发送消息处理完成");
            } else {
                println!("用户不存在: {}", user_id);
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
            user_id: _,
        } = msg
        {
            println!("处理加入房间的protobuf数据");
            // 暂时返回None，因为这是old_websocket，不应该被使用
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
            user_id: _,
        } = msg
        {
            println!("处理离开房间的protobuf数据");
            // 暂时返回None，因为这是old_websocket，不应该被使用
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
