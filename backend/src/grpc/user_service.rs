use crate::chat::{user_service_server::UserService, *};
use crate::database::{DbPool, UserRepository};
use crate::grpc::auth::AuthService;
use crate::models::{CreateUser, UpdateUser};
use crate::redis::SessionManager;
use redis::Client as RedisClient;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct UserServiceImpl {
    user_repo: UserRepository,
    session_manager: SessionManager,
    auth_service: AuthService,
}

impl UserServiceImpl {
    pub fn new(pool: DbPool, redis_client: RedisClient) -> Self {
        let user_repo = UserRepository::new(pool);
        let session_manager = SessionManager::new(redis_client);
        let auth_service = AuthService::new(
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
        );

        Self {
            user_repo,
            session_manager,
            auth_service,
        }
    }
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();

        // 检查用户是否已存在
        if let Some(_) = self
            .user_repo
            .find_by_email(&req.email)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?
        {
            return Ok(Response::new(RegisterResponse {
                success: false,
                message: "User already exists".to_string(),
                user: None,
            }));
        }

        let create_user = CreateUser {
            username: req.username,
            email: req.email,
            password: req.password,
        };

        let user = self
            .user_repo
            .create(create_user)
            .await
            .map_err(|e| Status::internal(format!("Failed to create user: {}", e)))?;

        Ok(Response::new(RegisterResponse {
            success: true,
            message: "User registered successfully".to_string(),
            user: Some(user.to_public().into()),
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?
            .ok_or_else(|| Status::not_found("User not found"))?;

        let is_valid = self
            .user_repo
            .verify_password(&user, &req.password)
            .await
            .map_err(|e| Status::internal(format!("Password verification failed: {}", e)))?;

        if !is_valid {
            return Ok(Response::new(LoginResponse {
                success: false,
                message: "Invalid password".to_string(),
                token: String::new(),
                user: None,
            }));
        }

        // 生成JWT token
        let token = self
            .auth_service
            .generate_token(user.id.clone(), user.username.clone())
            .map_err(|e| Status::internal(format!("Token generation failed: {}", e)))?;

        // 创建会话
        let _session_id = self
            .session_manager
            .create_session(user.id.clone(), user.username.clone())
            .await
            .map_err(|e| Status::internal(format!("Session creation failed: {}", e)))?;

        // 设置用户在线状态
        self.user_repo
            .set_online_status(&user.id, true)
            .await
            .map_err(|e| Status::internal(format!("Failed to set online status: {}", e)))?;

        self.session_manager
            .set_user_online(&user.id)
            .await
            .map_err(|e| Status::internal(format!("Failed to set user online: {}", e)))?;

        Ok(Response::new(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
            token,
            user: Some(user.to_public().into()),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();

        let user = self
            .user_repo
            .find_by_id(&req.user_id)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?
            .ok_or_else(|| Status::not_found("User not found"))?;

        Ok(Response::new(GetUserResponse {
            success: true,
            user: Some(user.to_public().into()),
        }))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let req = request.into_inner();

        let update_user = UpdateUser {
            username: if req.username.is_empty() {
                None
            } else {
                Some(req.username)
            },
            avatar: if req.avatar.is_empty() {
                None
            } else {
                Some(req.avatar)
            },
        };

        let user = self
            .user_repo
            .update(&req.user_id, update_user)
            .await
            .map_err(|e| Status::internal(format!("Failed to update user: {}", e)))?;

        Ok(Response::new(UpdateUserResponse {
            success: true,
            message: "User updated successfully".to_string(),
            user: Some(user.to_public().into()),
        }))
    }
}

impl From<crate::models::PublicUser> for User {
    fn from(user: crate::models::PublicUser) -> Self {
        User {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar: user.avatar.unwrap_or_default(),
            is_online: user.is_online.map(|v| v != 0).unwrap_or(false),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
