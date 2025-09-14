mod database;
mod grpc;
mod grpc_service;
mod http;
mod models;
mod redis;
mod websocket;

// 包含生成的gRPC代码
pub mod chat {
    tonic::include_proto!("chat");
}

use crate::chat::{chat_service_server::ChatServiceServer, user_service_server::UserServiceServer};
use database::{create_pool, init_database};
use grpc::{AuthService, UserServiceImpl};
use grpc_service::ChatServiceImpl as GrpcChatServiceImpl;
use http::create_routes;
use redis::{SessionManager, create_redis_client};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tracing::{error, info};
use warp::Filter;
use websocket::WebSocketHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载环境变量
    dotenv::dotenv().ok();

    info!("Starting Chat Server...");

    // 初始化数据库
    info!("Initializing database...");
    init_database().await?;
    let db_pool = create_pool().await?;
    info!("Database initialized successfully");

    // 初始化Redis
    info!("Initializing Redis...");
    let redis_client = create_redis_client().await?;
    let session_manager = SessionManager::new(redis_client.clone());
    info!("Redis initialized successfully");

    // 创建认证服务
    let auth_service = Arc::new(AuthService::new(
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
    ));

    // 创建服务实例
    let user_service = UserServiceImpl::new(db_pool.clone(), redis_client.clone());
    let message_repo = Arc::new(database::MessageRepository::new(db_pool.clone()));
    let chat_service = GrpcChatServiceImpl::new(message_repo);
    let ws_handler = Arc::new(WebSocketHandler::new(
        db_pool.clone(),
        session_manager.clone(),
    ));

    // 创建HTTP API路由
    let api_routes = create_routes(db_pool, session_manager, auth_service).with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]),
    );

    // 启动gRPC服务器（支持gRPC-Web）
    let grpc_addr = "0.0.0.0:50051".parse()?;
    let grpc_server = Server::builder()
        .accept_http1(true)
        .layer(
            tower::ServiceBuilder::new()
                .layer(
                    tower_http::cors::CorsLayer::new()
                        .allow_origin(tower_http::cors::Any)
                        .allow_headers(tower_http::cors::Any)
                        .allow_methods(tower_http::cors::Any),
                )
                .layer(GrpcWebLayer::new()),
        )
        .add_service(UserServiceServer::new(user_service))
        .add_service(ChatServiceServer::new(chat_service))
        .serve(grpc_addr);

    // 启动WebSocket服务器
    let ws_addr = "0.0.0.0:8301";
    let ws_listener = TcpListener::bind(ws_addr).await?;
    let ws_handler_clone = ws_handler.clone();

    let ws_server = tokio::spawn(async move {
        info!("WebSocket server listening on {}", ws_addr);
        while let Ok((stream, _)) = ws_listener.accept().await {
            let ws_handler = ws_handler_clone.clone();
            tokio::spawn(async move {
                ws_handler
                    .handle_connection(accept_async(stream).await.unwrap())
                    .await;
            });
        }
    });

    // 启动HTTP API服务器
    let health_route = warp::path("health").and(warp::get()).map(|| {
        warp::reply::json(&serde_json::json!({
            "status": "ok",
            "service": "chat-backend"
        }))
    });

    let all_routes = api_routes.or(health_route);
    let http_server = warp::serve(all_routes).run(([0, 0, 0, 0], 3001));

    info!("Server started successfully!");
    info!("gRPC server listening on {}", grpc_addr);
    info!("WebSocket server listening on {}", ws_addr);
    info!("HTTP health check server listening on 0.0.0.0:3001");

    // 运行所有服务器
    tokio::select! {
        result = grpc_server => {
            if let Err(e) = result {
                error!("gRPC server error: {}", e);
            }
        }
        result = ws_server => {
            if let Err(e) = result {
                error!("WebSocket server error: {}", e);
            }
        }
        _ = http_server => {
            info!("HTTP server stopped");
        }
    }

    Ok(())
}
