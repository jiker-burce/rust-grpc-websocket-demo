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
        message_context: &mut crate::websocket::new_websocket::event_handlers::MessageContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 确定消息类型
        let message_type = self.get_message_type(&message);

        // 获取对应的事件处理器
        if let Some(handler) = self.event_handler_factory.get_handler(&message_type) {
            // 更新消息处理上下文
            message_context.user_id = connection_state.get_user_id().clone();
            message_context.current_room = connection_state.get_current_room().clone();

            // 执行事件处理器
            let result = handler.handle(message, message_context).await?;

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
            WebSocketMessage::SendMessage { .. } => "send_message".to_string(),
            WebSocketMessage::JoinRoom { .. } => "join_room".to_string(),
            WebSocketMessage::LeaveRoom { .. } => "leave_room".to_string(),
            WebSocketMessage::GetMessages { .. } => "get_messages".to_string(),
            WebSocketMessage::MessagesResponse { .. } => "messages_response".to_string(),
            WebSocketMessage::GetOnlineUsers { .. } => "get_online_users".to_string(),
            WebSocketMessage::OnlineUsersResponse { .. } => "online_users_response".to_string(),
            WebSocketMessage::Error { .. } => "error".to_string(),
            WebSocketMessage::Success { .. } => "success".to_string(),
            // 实时通信消息
            WebSocketMessage::NewMessage { .. } => "new_message".to_string(),
            WebSocketMessage::UserJoined { .. } => "user_joined".to_string(),
            WebSocketMessage::UserLeft { .. } => "user_left".to_string(),
            WebSocketMessage::Ping { .. } => "ping".to_string(),
            WebSocketMessage::Pong { .. } => "pong".to_string(),
            WebSocketMessage::OnlineUsersList { .. } => "online_users_list".to_string(),
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
            MessageResult::SetUserIdAndRoomReceiver(user_id, receiver) => {
                connection_state.set_user_id(user_id);
                connection_state.set_room_receiver(receiver);
            }
            MessageResult::SendResponse(response) => {
                // 将WebSocketMessage转换为gRPC消息帧格式
                let grpc_frame = self.websocket_message_to_grpc_frame(&response);
                if let Ok(json) = serde_json::to_string(&grpc_frame) {
                    println!("发送响应消息: {}", json);
                    ws_sender.send(WsMessage::Text(json)).await?;
                }
            }
        }
        Ok(())
    }

    /// 将WebSocketMessage转换为gRPC消息帧格式
    fn websocket_message_to_grpc_frame(&self, message: &WebSocketMessage) -> serde_json::Value {
        match message {
            WebSocketMessage::MessagesResponse { data } => {
                serde_json::json!({
                    "type": "messages_response",
                    "data": data
                })
            }
            WebSocketMessage::OnlineUsersResponse { data } => {
                serde_json::json!({
                    "type": "online_users_response",
                    "data": data
                })
            }
            WebSocketMessage::Success { message } => {
                serde_json::json!({
                    "type": "success",
                    "message": message
                })
            }
            WebSocketMessage::Error { message } => {
                serde_json::json!({
                    "type": "error",
                    "message": message
                })
            }
            _ => {
                // 对于其他消息类型，尝试序列化为JSON
                serde_json::to_value(message).unwrap_or(serde_json::json!({
                    "type": "unknown",
                    "message": "无法序列化的消息类型"
                }))
            }
        }
    }
}
