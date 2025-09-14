use crate::websocket::WebSocketMessage;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 消息事件处理器的trait
#[async_trait::async_trait]
pub trait MessageEventHandler: Send + Sync {
    /// 处理消息事件
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>>;

    /// 获取处理器支持的消息类型
    fn supported_message_type(&self) -> &'static str;
}

/// 消息处理上下文
pub struct MessageContext {
    pub user_id: Option<String>,
    pub current_room: Option<String>,
    pub connection_id: Option<String>,
    pub broadcast_handler: Arc<Mutex<crate::websocket::BroadcastHandler>>,
}

/// 消息处理结果
pub enum MessageResult {
    /// 无操作
    NoOp,
    /// 设置用户ID
    SetUserId(String),
    /// 设置当前房间
    SetCurrentRoom(String),
    /// 清除当前房间
    ClearCurrentRoom,
    /// 设置房间接收器
    SetRoomReceiver(tokio::sync::broadcast::Receiver<WebSocketMessage>),
    /// 设置用户ID和房间接收器
    SetUserIdAndRoomReceiver(String, tokio::sync::broadcast::Receiver<WebSocketMessage>),
    /// 发送响应消息
    SendResponse(WebSocketMessage),
}

impl MessageContext {
    pub fn new(broadcast_handler: Arc<Mutex<crate::websocket::BroadcastHandler>>) -> Self {
        Self {
            user_id: None,
            current_room: None,
            connection_id: None,
            broadcast_handler,
        }
    }
}
