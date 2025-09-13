use super::EventHandlerFactory;
use super::event_handlers::{MessageContext, MessageEventHandlerEnum, MessageResult};
use crate::websocket::WebSocketMessage;
use futures_util::SinkExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message as WsMessage;

/// 命令处理器，负责执行消息处理命令
pub struct CommandProcessor {
    event_handler_factory: Arc<EventHandlerFactory>,
    broadcast_handler: Arc<Mutex<crate::websocket::BroadcastHandler>>,
}

impl CommandProcessor {
    pub fn new(
        event_handler_factory: Arc<EventHandlerFactory>,
        broadcast_handler: Arc<Mutex<crate::websocket::BroadcastHandler>>,
    ) -> Self {
        Self {
            event_handler_factory,
            broadcast_handler,
        }
    }

    /// 处理WebSocket消息
    pub async fn process_message(
        &self,
        message: WebSocketMessage,
        ws_sender: &mut futures_util::stream::SplitSink<
            WebSocketStream<tokio::net::TcpStream>,
            WsMessage,
        >,
        connection_state: &mut crate::websocket::ConnectionState,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 确定消息类型
        let message_type = self.get_message_type(&message);

        // 获取对应的事件处理器
        if let Some(handler) = self.event_handler_factory.get_handler(&message_type) {
            // 创建消息处理上下文
            let mut context = MessageContext::new(self.broadcast_handler.clone());
            context.user_id = connection_state.get_user_id().clone();
            context.current_room = connection_state.get_current_room().clone();

            // 执行事件处理器
            let result = handler.handle(message, &context).await?;

            // 处理事件处理器执行结果
            self.handle_event_handler_result(result, connection_state, ws_sender)
                .await?;
        } else {
            println!("未支持的消息类型: {}", message_type);
        }

        Ok(())
    }

    /// 获取消息类型
    fn get_message_type(&self, message: &WebSocketMessage) -> String {
        match message {
            WebSocketMessage::ChatMessage { .. } => "chat_message".to_string(),
            WebSocketMessage::JoinRoom { .. } => "join_room".to_string(),
            WebSocketMessage::LeaveRoom { .. } => "leave_room".to_string(),
            WebSocketMessage::Error { .. } => "error".to_string(),
            WebSocketMessage::UserOnline { .. } => "user_online".to_string(),
            WebSocketMessage::UserOffline { .. } => "user_offline".to_string(),
            WebSocketMessage::Success { .. } => "success".to_string(),
        }
    }

    /// 处理事件处理器执行结果
    async fn handle_event_handler_result(
        &self,
        result: MessageResult,
        connection_state: &mut crate::websocket::ConnectionState,
        ws_sender: &mut futures_util::stream::SplitSink<
            WebSocketStream<tokio::net::TcpStream>,
            WsMessage,
        >,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match result {
            MessageResult::NoOp => {
                // 无操作
            }
            MessageResult::SetUserId(user_id) => {
                connection_state.set_user_id(user_id);
            }
            MessageResult::SetCurrentRoom(room_id) => {
                connection_state.set_current_room(room_id);
            }
            MessageResult::ClearCurrentRoom => {
                connection_state.clear_current_room();
            }
            MessageResult::SetRoomReceiver(receiver) => {
                connection_state.set_room_receiver(receiver);
            }
            MessageResult::SendResponse(response) => {
                if let Ok(json) = response.to_json() {
                    ws_sender.send(WsMessage::Text(json)).await?;
                }
            }
        }
        Ok(())
    }
}
