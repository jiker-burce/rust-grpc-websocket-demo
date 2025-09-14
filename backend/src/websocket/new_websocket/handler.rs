use super::{CommandProcessor, EventHandlerFactory};
use crate::database::{DbPool, MessageRepository, UserRepository};
use crate::grpc::auth::AuthService;
use crate::redis::SessionManager;
use crate::websocket::new_websocket::event_handlers::MessageContext;
use crate::websocket::{BroadcastHandler, ConnectionState, UserTracker, WebSocketMessage};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message as WsMessage;

/// 重构后的WebSocket处理器，使用事件处理器+命令模式
pub struct WebSocketHandler {
    event_handler_factory: Arc<EventHandlerFactory>,
    broadcast_handler: Arc<tokio::sync::Mutex<BroadcastHandler>>,
    command_processor: Arc<CommandProcessor>,
    auth_service: AuthService,
    user_tracker: Arc<UserTracker>,
}

impl WebSocketHandler {
    pub fn new(pool: DbPool, session_manager: SessionManager) -> Self {
        let message_repo = Arc::new(MessageRepository::new(pool.clone()));
        let user_repo = Arc::new(UserRepository::new(pool));
        let auth_service = AuthService::new(
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
        );

        let session_manager_arc = Arc::new(session_manager);
        let broadcast_handler = Arc::new(tokio::sync::Mutex::new(BroadcastHandler::new()));
        let user_tracker = Arc::new(UserTracker::new());
        let event_handler_factory = Arc::new(EventHandlerFactory::new(
            user_repo.clone(),
            message_repo,
            session_manager_arc.clone(),
            broadcast_handler.clone(),
            user_tracker.clone(),
        ));
        let command_processor = Arc::new(CommandProcessor::new(
            event_handler_factory.clone(),
            broadcast_handler.clone(),
        ));

        Self {
            event_handler_factory,
            broadcast_handler,
            command_processor,
            auth_service,
            user_tracker,
        }
    }

    pub async fn handle_connection(
        &self,
        stream: WebSocketStream<tokio::net::TcpStream>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (mut ws_sender, mut ws_receiver) = stream.split();
        let mut connection_state = ConnectionState::new();

        // 生成唯一的连接ID
        let connection_id = uuid::Uuid::new_v4().to_string();
        println!("新WebSocket连接建立: connection_id={}", connection_id);

        // 创建消息上下文并设置连接ID
        let mut message_context = MessageContext::new(self.broadcast_handler.clone());
        message_context.connection_id = Some(connection_id.clone());

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
                            &mut message_context,
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

        // 连接结束，清理用户状态
        if let Some((user_id, room_id)) = self
            .user_tracker
            .connection_disconnect(connection_id.clone())
            .await
        {
            println!(
                "连接断开，清理用户状态: connection_id={}, user_id={}, room_id={}",
                connection_id, user_id, room_id
            );

            // 广播用户离开消息
            let broadcast_message = WebSocketMessage::UserLeft {
                user_id: user_id.clone(),
                username: format!("用户_{}", &user_id[5..13]),
                room_id: room_id.clone(),
            };

            let mut broadcast_handler = self.broadcast_handler.lock().await;
            broadcast_handler.broadcast_to_room(&room_id, &broadcast_message);

            // 广播更新后的用户列表
            let room_users = self.user_tracker.get_room_users(&room_id).await;
            let users_list_message = WebSocketMessage::OnlineUsersList {
                room_id: room_id.clone(),
                users: room_users,
            };
            broadcast_handler.broadcast_to_room(&room_id, &users_list_message);
        } else {
            println!(
                "连接断开，未找到对应的用户: connection_id={}",
                connection_id
            );
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
        message_context: &mut MessageContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match msg {
            WsMessage::Text(text) => {
                println!("收到WebSocket文本消息: {}", text);

                let ws_msg: WebSocketMessage = serde_json::from_str(&text)?;
                println!("成功解析WebSocket消息: {:?}", ws_msg);

                // 使用命令处理器处理消息
                self.command_processor
                    .process_message(ws_msg, ws_sender, connection_state, message_context)
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
