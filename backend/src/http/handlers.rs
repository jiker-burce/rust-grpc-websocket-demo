use crate::database::{DbPool, MessageRepository, UserRepository};
use crate::grpc::auth::AuthService;
use crate::models::{CreateUser, UpdateUser};
use crate::redis::SessionManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub room_id: String,
    pub message_type: Option<String>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

pub fn create_routes(
    pool: DbPool,
    session_manager: SessionManager,
    auth_service: Arc<AuthService>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool));

    // 用户路由
    let user_routes = user_routes(
        user_repo.clone(),
        session_manager.clone(),
        auth_service.clone(),
    );

    // 聊天路由
    let chat_routes = chat_routes(user_repo, message_repo, session_manager, auth_service);

    user_routes.or(chat_routes)
}

fn user_routes(
    user_repo: Arc<UserRepository>,
    session_manager: SessionManager,
    auth_service: Arc<AuthService>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let register = warp::path("api")
        .and(warp::path("users"))
        .and(warp::path("register"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_user_repo(user_repo.clone()))
        .and_then(handle_register);

    let login = warp::path("api")
        .and(warp::path("users"))
        .and(warp::path("login"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_user_repo(user_repo.clone()))
        .and(with_session_manager(session_manager))
        .and(with_auth_service(auth_service))
        .and_then(handle_login);

    register.or(login)
}

fn chat_routes(
    user_repo: Arc<UserRepository>,
    message_repo: Arc<MessageRepository>,
    session_manager: SessionManager,
    auth_service: Arc<AuthService>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let send_message = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::path("messages"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_user_repo(user_repo.clone()))
        .and(with_message_repo(message_repo))
        .and(with_session_manager(session_manager.clone()))
        .and(with_auth_service(auth_service.clone()))
        .and_then(handle_send_message);

    let get_messages = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::path("rooms"))
        .and(warp::path::param::<String>())
        .and(warp::path("messages"))
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_message_repo(Arc::new(MessageRepository::new(
            user_repo.pool().clone(),
        ))))
        .and_then(handle_get_messages);

    let get_online_users = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::path("rooms"))
        .and(warp::path::param::<String>())
        .and(warp::path("users"))
        .and(warp::get())
        .and(with_session_manager(session_manager.clone()))
        .and_then(handle_get_online_users);

    let join_room = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::path("rooms"))
        .and(warp::path::param::<String>())
        .and(warp::path("join"))
        .and(warp::post())
        .and(with_session_manager(session_manager.clone()))
        .and_then(handle_join_room);

    let leave_room = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::path("rooms"))
        .and(warp::path::param::<String>())
        .and(warp::path("leave"))
        .and(warp::post())
        .and(with_session_manager(session_manager.clone()))
        .and_then(handle_leave_room);

    let get_rooms = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::path("rooms"))
        .and(warp::get())
        .and(with_user_repo(user_repo))
        .and_then(handle_get_rooms);

    send_message
        .or(get_messages)
        .or(get_online_users)
        .or(join_room)
        .or(leave_room)
        .or(get_rooms)
}

// 辅助函数来传递依赖
fn with_user_repo(
    user_repo: Arc<UserRepository>,
) -> impl Filter<Extract = (Arc<UserRepository>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || user_repo.clone())
}

fn with_message_repo(
    message_repo: Arc<MessageRepository>,
) -> impl Filter<Extract = (Arc<MessageRepository>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || message_repo.clone())
}

fn with_session_manager(
    session_manager: SessionManager,
) -> impl Filter<Extract = (SessionManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || session_manager.clone())
}

fn with_auth_service(
    auth_service: Arc<AuthService>,
) -> impl Filter<Extract = (Arc<AuthService>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || auth_service.clone())
}

// 处理函数
async fn handle_register(
    req: RegisterRequest,
    user_repo: Arc<UserRepository>,
) -> Result<impl Reply, Rejection> {
    let create_user = CreateUser {
        username: req.username,
        email: req.email,
        password: req.password,
    };

    match user_repo.create(create_user).await {
        Ok(user) => Ok(warp::reply::json(&ApiResponse::success(
            user.to_public(),
            "注册成功",
        ))),
        Err(e) => Ok(warp::reply::json(&ApiResponse::<()>::error(&format!(
            "注册失败: {}",
            e
        )))),
    }
}

async fn handle_login(
    req: LoginRequest,
    user_repo: Arc<UserRepository>,
    session_manager: SessionManager,
    auth_service: Arc<AuthService>,
) -> Result<impl Reply, Rejection> {
    match user_repo.find_by_email(&req.email).await {
        Ok(Some(user)) => {
            match user_repo.verify_password(&user, &req.password).await {
                Ok(true) => {
                    // 生成JWT token
                    match auth_service.generate_token(user.id.clone(), user.username.clone()) {
                        Ok(token) => {
                            // 创建会话
                            let _ = session_manager
                                .create_session(user.id.clone(), user.username.clone())
                                .await;

                            // 设置用户在线状态
                            let _ = user_repo.set_online_status(&user.id, true).await;
                            let _ = session_manager.set_user_online(&user.id).await;

                            #[derive(Serialize)]
                            struct LoginResponse {
                                user: crate::models::PublicUser,
                                token: String,
                            }

                            Ok(warp::reply::json(&ApiResponse::success(
                                LoginResponse {
                                    user: user.to_public(),
                                    token,
                                },
                                "登录成功",
                            )))
                        }
                        Err(_) => Ok(warp::reply::json(&ApiResponse::<()>::error(
                            "Token生成失败",
                        ))),
                    }
                }
                Ok(false) => Ok(warp::reply::json(&ApiResponse::<()>::error("密码错误"))),
                Err(_) => Ok(warp::reply::json(&ApiResponse::<()>::error("密码验证失败"))),
            }
        }
        Ok(None) => Ok(warp::reply::json(&ApiResponse::<()>::error("用户不存在"))),
        Err(_) => Ok(warp::reply::json(&ApiResponse::<()>::error("数据库错误"))),
    }
}

async fn handle_send_message(
    req: SendMessageRequest,
    user_repo: Arc<UserRepository>,
    message_repo: Arc<MessageRepository>,
    session_manager: SessionManager,
    auth_service: Arc<AuthService>,
) -> Result<impl Reply, Rejection> {
    // 这里需要从请求中获取用户ID，实际实现中应该从JWT token中解析
    // 为了简化，这里假设用户ID在请求中
    let user_id = "temp_user_id".to_string(); // 实际应该从认证中间件获取

    match user_repo.find_by_id(&user_id).await {
        Ok(Some(user)) => {
            let message_type = match req.message_type.as_deref() {
                Some("image") => crate::models::MessageType::Image,
                Some("file") => crate::models::MessageType::File,
                _ => crate::models::MessageType::Text,
            };

            let message = crate::models::Message::new(
                user_id,
                user.username,
                req.content,
                req.room_id,
                message_type,
            );

            match message_repo.create(message).await {
                Ok(saved_message) => Ok(warp::reply::json(&ApiResponse::success(
                    saved_message.to_grpc(),
                    "消息发送成功",
                ))),
                Err(e) => Ok(warp::reply::json(&ApiResponse::<()>::error(&format!(
                    "消息发送失败: {}",
                    e
                )))),
            }
        }
        Ok(None) => Ok(warp::reply::json(&ApiResponse::<()>::error("用户不存在"))),
        Err(_) => Ok(warp::reply::json(&ApiResponse::<()>::error("数据库错误"))),
    }
}

async fn handle_get_messages(
    room_id: String,
    query: std::collections::HashMap<String, String>,
    message_repo: Arc<MessageRepository>,
) -> Result<impl Reply, Rejection> {
    let limit = query
        .get("limit")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1000); // 增加默认限制到1000条消息

    let before_timestamp = query
        .get("before_timestamp")
        .and_then(|s| s.parse::<i64>().ok());

    println!(
        "获取消息请求 - 房间ID: {}, 限制: {}, before_timestamp: {:?}",
        room_id, limit, before_timestamp
    );

    match message_repo
        .get_messages_by_room(&room_id, limit, before_timestamp)
        .await
    {
        Ok(messages) => {
            println!("从数据库获取到 {} 条消息", messages.len());
            let grpc_messages: Vec<_> = messages.into_iter().map(|m| m.to_grpc()).collect();
            Ok(warp::reply::json(&grpc_messages))
        }
        Err(e) => Ok(warp::reply::json(&ApiResponse::<()>::error(&format!(
            "获取消息失败: {}",
            e
        )))),
    }
}

async fn handle_get_online_users(
    room_id: String,
    session_manager: SessionManager,
) -> Result<impl Reply, Rejection> {
    match session_manager.get_room_users(&room_id).await {
        Ok(user_ids) => {
            // 获取用户详细信息
            let pool = sqlx::MySqlPool::connect(
                &std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "mysql://root:123456@localhost:3306/chat_db".to_string()),
            )
            .await
            .map_err(|_| warp::reject::reject())?;

            let user_repo = UserRepository::new(pool);
            let mut users = Vec::new();
            for user_id in user_ids {
                if let Ok(Some(user)) = user_repo.find_by_id(&user_id).await {
                    users.push(user);
                }
            }

            #[derive(Serialize)]
            struct OnlineUsersResponse {
                users: Vec<crate::models::User>,
            }
            Ok(warp::reply::json(&OnlineUsersResponse { users }))
        }
        Err(e) => Ok(warp::reply::json(&ApiResponse::<()>::error(&format!(
            "获取在线用户失败: {}",
            e
        )))),
    }
}

async fn handle_join_room(
    room_id: String,
    session_manager: SessionManager,
) -> Result<impl Reply, Rejection> {
    // 这里应该从请求头或JWT token中获取用户ID
    // 为了简化，我们暂时使用一个固定的用户ID
    let user_id = "temp_user_id".to_string();

    match session_manager.add_user_to_room(&room_id, &user_id).await {
        Ok(_) => Ok(warp::reply::json(&ApiResponse::success((), "成功加入房间"))),
        Err(e) => Ok(warp::reply::json(&ApiResponse::<()>::error(&format!(
            "加入房间失败: {}",
            e
        )))),
    }
}

async fn handle_leave_room(
    room_id: String,
    session_manager: SessionManager,
) -> Result<impl Reply, Rejection> {
    // 这里应该从请求头或JWT token中获取用户ID
    let user_id = "temp_user_id".to_string();

    match session_manager
        .remove_user_from_room(&room_id, &user_id)
        .await
    {
        Ok(_) => Ok(warp::reply::json(&ApiResponse::success((), "成功离开房间"))),
        Err(e) => Ok(warp::reply::json(&ApiResponse::<()>::error(&format!(
            "离开房间失败: {}",
            e
        )))),
    }
}

async fn handle_get_rooms(user_repo: Arc<UserRepository>) -> Result<impl Reply, Rejection> {
    // 返回默认的房间列表
    let rooms = vec![
        serde_json::json!({
            "id": "general",
            "name": "公共聊天室",
            "description": "所有人都可以加入的公共聊天室",
            "is_public": true,
            "user_count": 0
        }),
        serde_json::json!({
            "id": "tech",
            "name": "技术讨论",
            "description": "技术相关的讨论房间",
            "is_public": true,
            "user_count": 0
        }),
        serde_json::json!({
            "id": "random",
            "name": "闲聊",
            "description": "随意聊天的房间",
            "is_public": true,
            "user_count": 0
        }),
    ];

    Ok(warp::reply::json(&ApiResponse::success(
        rooms,
        "获取房间列表成功",
    )))
}
