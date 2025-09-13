use crate::database::{DbPool, MessageRepository, UserRepository};
use crate::grpc::auth::AuthService;
use crate::redis::SessionManager;
use crate::websocket::{
    BroadcastHandler, CommandProcessor, ConnectionState, StrategyFactory, WebSocketMessage,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message as WsMessage;

/// 重构后的WebSocket处理器，使用策略模式+命令模式
pub struct WebSocketHandlerV2 {
    strategy_factory: Arc<StrategyFactory>,
    broadcast_handler: Arc<tokio::sync::Mutex<BroadcastHandler>>,
    command_processor: Arc<CommandProcessor>,
    auth_service: AuthService,
}

impl WebSocketHandlerV2 {
    pub fn new(pool: DbPool, session_manager: SessionManager) -> Self {
        let message_repo = Arc::new(MessageRepository::new(pool.clone()));
        let user_repo = Arc::new(UserRepository::new(pool));
        let auth_service = AuthService::new(
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
        );

        let strategy_factory = Arc::new(StrategyFactory::new(user_repo.clone(), message_repo));
        let broadcast_handler = Arc::new(tokio::sync::Mutex::new(BroadcastHandler::new()));
        let command_processor = Arc::new(CommandProcessor::new(
            strategy_factory.clone(),
            broadcast_handler.clone(),
        ));

        Self {
            strategy_factory,
            broadcast_handler,
            command_processor,
            auth_service,
        }
    }

    pub async fn handle_connection(
        &self,
        stream: WebSocketStream<tokio::net::TcpStream>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (mut ws_sender, mut ws_receiver) = stream.split();
        let mut connection_state = ConnectionState::new();

        // 主消息处理循环
        loop {
            tokio::select! {
                // 处理从客户端接收的消息
                msg = ws_receiver.next() => {
                    if let Some(msg) = msg {
                        if let Err(e) = self.handle_websocket_message(
                            msg?,
                            &mut ws_sender,
                            &mut connection_state,
                        ).await {
                            println!("处理WebSocket消息失败: {}", e);
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // 处理房间广播消息
                broadcast_msg = async {
                    if let Some(ref mut receiver) = connection_state.room_receiver {
                        receiver.recv().await
                    } else {
                        std::future::pending().await
                    }
                } => {
                    if let Ok(msg) = broadcast_msg {
                        if self.should_send_broadcast(&msg, connection_state.get_user_id()) {
                            if let Err(e) = self.send_message_to_client(&mut ws_sender, &msg).await {
                                println!("发送广播消息失败: {}", e);
                                break;
                            }
                        }
                    } else {
                        // 广播通道关闭，重新获取接收器
                        if let Some(room_id) = connection_state.get_current_room() {
                            let mut broadcast_handler = self.broadcast_handler.lock().await;
                            if let Some(receiver) = broadcast_handler.get_room_receiver(room_id) {
                                connection_state.set_room_receiver(receiver);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 处理WebSocket消息 - 现在变得非常简洁
    async fn handle_websocket_message(
        &self,
        msg: WsMessage,
        ws_sender: &mut futures_util::stream::SplitSink<
            WebSocketStream<tokio::net::TcpStream>,
            WsMessage,
        >,
        connection_state: &mut ConnectionState,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match msg {
            WsMessage::Text(text) => {
                println!("收到WebSocket文本消息: {}", text);

                let ws_msg: WebSocketMessage = serde_json::from_str(&text)?;
                println!("成功解析WebSocket消息: {:?}", ws_msg);

                // 使用命令处理器处理消息
                self.command_processor
                    .process_message(ws_msg, ws_sender, connection_state)
                    .await?;
            }
            WsMessage::Close(_) => {
                println!("WebSocket连接关闭");
                return Err("连接关闭".into());
            }
            _ => {}
        }
        Ok(())
    }

    /// 判断是否应该发送广播消息给客户端
    fn should_send_broadcast(
        &self,
        msg: &WebSocketMessage,
        current_user_id: &Option<String>,
    ) -> bool {
        let broadcast_handler = self.broadcast_handler.try_lock();
        if let Ok(handler) = broadcast_handler {
            handler.should_send_to_client(msg, current_user_id)
        } else {
            true // 如果无法获取锁，默认发送
        }
    }

    /// 发送消息到客户端
    async fn send_message_to_client(
        &self,
        ws_sender: &mut futures_util::stream::SplitSink<
            WebSocketStream<tokio::net::TcpStream>,
            WsMessage,
        >,
        message: &WebSocketMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Ok(json) = message.to_json() {
            ws_sender.send(WsMessage::Text(json)).await?;
        }
        Ok(())
    }
}
