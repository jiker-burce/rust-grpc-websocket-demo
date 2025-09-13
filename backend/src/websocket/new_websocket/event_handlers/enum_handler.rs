use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::websocket::WebSocketMessage;

/// 使用枚举实现的事件处理器，避免trait对象的问题
pub enum MessageEventHandlerEnum {
    ChatMessage(ChatMessageHandler),
    JoinRoom(JoinRoomHandler),
    LeaveRoom(LeaveRoomHandler),
    Error(ErrorHandler),
}

impl MessageEventHandlerEnum {
    pub async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            MessageEventHandlerEnum::ChatMessage(handler) => handler.handle(message, context).await,
            MessageEventHandlerEnum::JoinRoom(handler) => handler.handle(message, context).await,
            MessageEventHandlerEnum::LeaveRoom(handler) => handler.handle(message, context).await,
            MessageEventHandlerEnum::Error(handler) => handler.handle(message, context).await,
        }
    }

    pub fn supported_message_type(&self) -> &'static str {
        match self {
            MessageEventHandlerEnum::ChatMessage(handler) => handler.supported_message_type(),
            MessageEventHandlerEnum::JoinRoom(handler) => handler.supported_message_type(),
            MessageEventHandlerEnum::LeaveRoom(handler) => handler.supported_message_type(),
            MessageEventHandlerEnum::Error(handler) => handler.supported_message_type(),
        }
    }
}

// 重新导出事件处理器类型
use super::{ChatMessageHandler, ErrorHandler, JoinRoomHandler, LeaveRoomHandler};
