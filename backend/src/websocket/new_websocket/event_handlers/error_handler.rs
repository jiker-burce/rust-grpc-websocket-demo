use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::websocket::WebSocketMessage;

/// 错误消息事件处理器
pub struct ErrorHandler;

impl ErrorHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for ErrorHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        _context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::Error { message: error_msg } = message {
            println!("收到错误消息: {}", error_msg);
        }
        Ok(MessageResult::NoOp)
    }

    fn supported_message_type(&self) -> &'static str {
        "error"
    }
}
