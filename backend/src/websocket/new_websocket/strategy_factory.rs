use super::event_handlers::{
    ChatMessageHandler, ErrorHandler, JoinRoomHandler, LeaveRoomHandler, MessageEventHandlerEnum,
};
use crate::database::{MessageRepository, UserRepository};
use crate::redis::SessionManager;
use std::collections::HashMap;
use std::sync::Arc;

/// 事件处理器工厂，负责创建和管理消息事件处理器
pub struct EventHandlerFactory {
    handlers: HashMap<String, MessageEventHandlerEnum>,
}

impl EventHandlerFactory {
    pub fn new(
        user_repo: Arc<UserRepository>,
        message_repo: Arc<MessageRepository>,
        session_manager: Arc<SessionManager>,
    ) -> Self {
        let mut handlers: HashMap<String, MessageEventHandlerEnum> = HashMap::new();

        // 注册各种消息事件处理器
        handlers.insert(
            "chat_message".to_string(),
            MessageEventHandlerEnum::ChatMessage(ChatMessageHandler::new(
                user_repo.clone(),
                message_repo.clone(),
            )),
        );

        handlers.insert(
            "join_room".to_string(),
            MessageEventHandlerEnum::JoinRoom(JoinRoomHandler::new(
                user_repo.clone(),
                session_manager.clone(),
            )),
        );

        handlers.insert(
            "leave_room".to_string(),
            MessageEventHandlerEnum::LeaveRoom(LeaveRoomHandler::new(session_manager.clone())),
        );

        handlers.insert(
            "error".to_string(),
            MessageEventHandlerEnum::Error(ErrorHandler::new()),
        );

        Self { handlers }
    }

    /// 根据消息类型获取对应的事件处理器
    pub fn get_handler(&self, message_type: &str) -> Option<&MessageEventHandlerEnum> {
        self.handlers.get(message_type)
    }

    /// 获取所有支持的消息类型
    pub fn get_supported_message_types(&self) -> Vec<&str> {
        self.handlers.keys().map(|s| s.as_str()).collect()
    }
}
