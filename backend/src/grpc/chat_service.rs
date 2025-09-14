use crate::chat::{chat_service_server::ChatService, *};
use crate::database::{DbPool, MessageRepository, UserRepository};
use crate::grpc::auth::AuthService;
use crate::models::{Message, MessageType};
use crate::redis::SessionManager;
use redis::Client as RedisClient;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tonic::{Request, Response, Status, Streaming};

pub struct ChatServiceImpl {
    message_repo: MessageRepository,
    user_repo: UserRepository,
    session_manager: SessionManager,
    auth_service: AuthService,
    // 广播通道用于实时消息推送
    message_senders: Arc<tokio::sync::Mutex<HashMap<String, broadcast::Sender<ChatMessage>>>>,
}

impl ChatServiceImpl {
    pub fn new(pool: DbPool, redis_client: RedisClient) -> Self {
        let message_repo = MessageRepository::new(pool.clone());
        let user_repo = UserRepository::new(pool);
        let session_manager = SessionManager::new(redis_client);
        let auth_service = AuthService::new(
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
        );

        Self {
            message_repo,
            user_repo,
            session_manager,
            auth_service,
            message_senders: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    fn get_or_create_room_sender(&self, room_id: &str) -> broadcast::Sender<ChatMessage> {
        let mut senders = futures::executor::block_on(self.message_senders.lock());
        senders
            .entry(room_id.to_string())
            .or_insert_with(|| broadcast::channel(1000).0)
            .clone()
    }
}

#[tonic::async_trait]
impl ChatService for ChatServiceImpl {
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();

        // 获取用户信息
        let user = self
            .user_repo
            .find_by_id(&req.user_id)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?
            .ok_or_else(|| Status::not_found("User not found"))?;

        // 创建消息
        let room_id = req.room_id.clone();
        let message = Message::new(
            req.user_id,
            user.username,
            req.content,
            req.room_id,
            MessageType::from(req.message_type),
        );

        // 保存消息到数据库
        let saved_message = self
            .message_repo
            .create(message)
            .await
            .map_err(|e| Status::internal(format!("Failed to save message: {}", e)))?;

        // 广播消息到房间
        let grpc_message = saved_message.to_grpc();
        let sender = self.get_or_create_room_sender(&room_id);
        let _ = sender.send(grpc_message.clone());

        Ok(Response::new(SendMessageResponse {
            success: true,
            message: "Message sent successfully".to_string(),
            chat_message: Some(grpc_message),
        }))
    }

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        let req = request.into_inner();

        // 获取历史消息
        let messages = self
            .message_repo
            .get_messages_by_room(
                &req.room_id,
                req.limit,
                if req.before_timestamp > 0 {
                    Some(req.before_timestamp)
                } else {
                    None
                },
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to get messages: {}", e)))?;

        // 转换为gRPC消息
        let grpc_messages: Vec<ChatMessage> =
            messages.into_iter().map(|msg| msg.to_grpc()).collect();

        // 创建响应
        let response = GetMessagesResponse {
            messages: grpc_messages,
        };

        Ok(Response::new(response))
    }

    async fn get_online_users(
        &self,
        request: Request<GetOnlineUsersRequest>,
    ) -> Result<Response<GetOnlineUsersResponse>, Status> {
        let req = request.into_inner();

        // 从Redis获取房间在线用户
        let room_users = self
            .session_manager
            .get_room_users(&req.room_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get room users: {}", e)))?;

        let mut users = Vec::new();
        for user_id in room_users {
            if let Some(user) = self
                .user_repo
                .find_by_id(&user_id)
                .await
                .map_err(|e| Status::internal(format!("Database error: {}", e)))?
            {
                users.push(user.to_public().into());
            }
        }

        Ok(Response::new(GetOnlineUsersResponse { users }))
    }

    async fn join_room(
        &self,
        request: Request<JoinRoomRequest>,
    ) -> Result<Response<JoinRoomResponse>, Status> {
        let req = request.into_inner();

        // 将用户添加到房间
        self.session_manager
            .add_user_to_room(&req.user_id, &req.room_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to join room: {}", e)))?;

        // 更新用户会话中的房间信息
        if let Some(session) = self
            .session_manager
            .get_session(&req.user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get session: {}", e)))?
        {
            self.session_manager
                .update_session_room(&req.user_id, Some(req.room_id.clone()))
                .await
                .map_err(|e| Status::internal(format!("Failed to update session: {}", e)))?;
        }

        Ok(Response::new(JoinRoomResponse {
            success: true,
            message: "Joined room successfully".to_string(),
        }))
    }

    async fn leave_room(
        &self,
        request: Request<LeaveRoomRequest>,
    ) -> Result<Response<LeaveRoomResponse>, Status> {
        let req = request.into_inner();

        // 从房间移除用户
        self.session_manager
            .remove_user_from_room(&req.user_id, &req.room_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to leave room: {}", e)))?;

        // 更新用户会话
        self.session_manager
            .update_session_room(&req.user_id, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to update session: {}", e)))?;

        Ok(Response::new(LeaveRoomResponse {
            success: true,
            message: "Left room successfully".to_string(),
        }))
    }
}
