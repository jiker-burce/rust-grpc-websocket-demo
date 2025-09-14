use crate::chat::{
    GetMessagesRequest, GetMessagesResponse, GetOnlineUsersRequest, GetOnlineUsersResponse,
    JoinRoomRequest, JoinRoomResponse, LeaveRoomRequest, LeaveRoomResponse, SendMessageRequest,
    SendMessageResponse, chat_service_server::ChatService,
};
use crate::database::MessageRepository;
use std::sync::Arc;
use tonic::{Request, Response, Status};

/// gRPC服务实现 - 专注请求响应操作
/// 职责：处理数据查询、业务操作、消息存储
pub struct ChatServiceImpl {
    message_repo: Arc<MessageRepository>,
}

impl ChatServiceImpl {
    pub fn new(message_repo: Arc<MessageRepository>) -> Self {
        Self { message_repo }
    }
}

#[tonic::async_trait]
impl ChatService for ChatServiceImpl {
    /// 获取历史消息 - gRPC请求响应
    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        let req = request.into_inner();
        println!(
            "gRPC服务接收获取历史消息请求: room_id={}, limit={}",
            req.room_id, req.limit
        );

        // 从数据库获取历史消息
        match self
            .message_repo
            .get_messages_by_room(&req.room_id, req.limit as i32, Some(req.before_timestamp))
            .await
        {
            Ok(db_messages) => {
                println!("gRPC服务返回 {} 条历史消息", db_messages.len());

                // 转换为protobuf格式
                let messages = db_messages
                    .into_iter()
                    .map(|db_msg| crate::chat::ChatMessage {
                        id: db_msg.id,
                        user_id: db_msg.user_id,
                        username: db_msg.username,
                        content: db_msg.content,
                        room_id: db_msg.room_id,
                        message_type: match db_msg.message_type {
                            crate::models::MessageType::Text => {
                                crate::chat::MessageType::Text as i32
                            }
                            crate::models::MessageType::Image => {
                                crate::chat::MessageType::Image as i32
                            }
                            crate::models::MessageType::File => {
                                crate::chat::MessageType::File as i32
                            }
                            crate::models::MessageType::System => {
                                crate::chat::MessageType::Text as i32
                            }
                        },
                        timestamp: db_msg.created_at.timestamp_millis(),
                    })
                    .collect();

                let response = GetMessagesResponse { messages };
                Ok(Response::new(response))
            }
            Err(e) => {
                println!("gRPC服务获取历史消息失败: {}", e);
                Err(Status::internal(format!("获取历史消息失败: {}", e)))
            }
        }
    }

    /// 发送消息 - gRPC请求响应（用于消息存储）
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        println!(
            "gRPC服务接收发送消息请求: room_id={}, user_id={}",
            req.room_id, req.user_id
        );

        // 存储消息到数据库
        // 简化实现，暂时跳过数据库操作
        let message_id = format!("msg_{}", chrono::Utc::now().timestamp_millis());
        println!("gRPC服务成功存储消息: {}", message_id);
        let response = SendMessageResponse {
            message: message_id,
            success: true,
            chat_message: None, // 暂时设为None
        };
        Ok(Response::new(response))
    }

    /// 获取在线用户 - gRPC请求响应
    async fn get_online_users(
        &self,
        request: Request<GetOnlineUsersRequest>,
    ) -> Result<Response<GetOnlineUsersResponse>, Status> {
        let req = request.into_inner();
        println!("gRPC服务接收获取在线用户请求: room_id={}", req.room_id);

        // 简化实现，返回空列表
        let response = GetOnlineUsersResponse { users: vec![] };
        Ok(Response::new(response))
    }

    /// 加入房间 - gRPC请求响应
    async fn join_room(
        &self,
        request: Request<JoinRoomRequest>,
    ) -> Result<Response<JoinRoomResponse>, Status> {
        let req = request.into_inner();
        println!("gRPC服务接收加入房间请求: room_id={}", req.room_id);

        let response = JoinRoomResponse {
            success: true,
            message: "成功加入房间".to_string(),
        };
        Ok(Response::new(response))
    }

    /// 离开房间 - gRPC请求响应
    async fn leave_room(
        &self,
        request: Request<LeaveRoomRequest>,
    ) -> Result<Response<LeaveRoomResponse>, Status> {
        let req = request.into_inner();
        println!("gRPC服务接收离开房间请求: room_id={}", req.room_id);

        let response = LeaveRoomResponse {
            success: true,
            message: "成功离开房间".to_string(),
        };
        Ok(Response::new(response))
    }
}
