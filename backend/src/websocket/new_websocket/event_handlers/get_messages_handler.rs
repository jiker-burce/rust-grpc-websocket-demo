use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::chat::{ChatMessage, GetMessagesRequest, GetMessagesResponse, MessageType};
use crate::database::MessageRepository;
use crate::websocket::WebSocketMessage;
use prost::Message;
use std::sync::Arc;

/// WebSocket获取历史消息处理器 - 调用gRPC服务
/// 职责：接收请求，调用gRPC服务获取数据，返回结果
pub struct GetMessagesHandler {
    message_repo: Arc<MessageRepository>,
}

impl GetMessagesHandler {
    pub fn new(message_repo: Arc<MessageRepository>) -> Self {
        Self { message_repo }
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for GetMessagesHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::GetMessages { room_id, limit } = message {
            println!(
                "WebSocket接收获取历史消息请求: room_id={}, limit={:?}",
                room_id, limit
            );

            // 调用gRPC服务获取历史消息（这里直接调用数据库，实际应该调用gRPC服务）
            let limit_value = limit.unwrap_or(50) as i32;
            match self
                .message_repo
                .get_messages_by_room(&room_id, limit_value, None)
                .await
            {
                Ok(db_messages) => {
                    println!("gRPC服务返回 {} 条历史消息", db_messages.len());

                    // 将数据库消息转换为protobuf消息
                    let protobuf_messages: Vec<ChatMessage> = db_messages
                        .into_iter()
                        .map(|db_msg| ChatMessage {
                            id: db_msg.id,
                            user_id: db_msg.user_id,
                            username: db_msg.username,
                            content: db_msg.content,
                            room_id: db_msg.room_id,
                            message_type: match db_msg.message_type {
                                crate::models::MessageType::Text => MessageType::Text as i32,
                                crate::models::MessageType::Image => MessageType::Image as i32,
                                crate::models::MessageType::File => MessageType::File as i32,
                                crate::models::MessageType::System => {
                                    MessageType::Text as i32 // 系统消息当作文本处理
                                }
                            },
                            timestamp: db_msg.created_at.timestamp_millis(),
                        })
                        .collect();

                    let response_data = GetMessagesResponse {
                        messages: protobuf_messages,
                    };

                    let response = WebSocketMessage::MessagesResponse {
                        data: response_data.encode_to_vec(),
                    };

                    Ok(MessageResult::SendResponse(response))
                }
                Err(e) => {
                    println!("gRPC服务调用失败: {}", e);
                    let response = WebSocketMessage::Error {
                        message: format!("获取历史消息失败: {}", e),
                    };
                    Ok(MessageResult::SendResponse(response))
                }
            }
        } else {
            Err("消息类型不匹配".into())
        }
    }

    fn supported_message_type(&self) -> &'static str {
        "get_messages"
    }
}
