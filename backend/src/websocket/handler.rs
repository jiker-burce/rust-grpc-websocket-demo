use crate::database::{DbPool, MessageRepository, UserRepository};
use crate::grpc::auth::AuthService;
use crate::models::{Message, MessageType};
use crate::redis::SessionManager;
use crate::websocket::WebSocketMessage;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{WebSocketStream, accept_async};

pub struct WebSocketHandler {
    message_repo: MessageRepository,
    user_repo: UserRepository,
    session_manager: SessionManager,
    auth_service: AuthService,
    room_senders: Arc<tokio::sync::Mutex<HashMap<String, broadcast::Sender<WebSocketMessage>>>>,
}

impl WebSocketHandler {
    pub fn new(pool: DbPool, session_manager: SessionManager) -> Self {
        let message_repo = MessageRepository::new(pool.clone());
        let user_repo = UserRepository::new(pool);
        let auth_service = AuthService::new(
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
        );

        Self {
            message_repo,
            user_repo,
            session_manager,
            auth_service,
            room_senders: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    pub async fn handle_connection(
        &self,
        stream: WebSocketStream<tokio::net::TcpStream>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (mut ws_sender, mut ws_receiver) = stream.split();
        let mut user_id: Option<String> = None;
        let mut current_room: Option<String> = None;
        let mut room_receiver: Option<broadcast::Receiver<WebSocketMessage>> = None;

        // 处理WebSocket消息
        loop {
            tokio::select! {
                // 处理从客户端接收的消息
                msg = ws_receiver.next() => {
                    if let Some(msg) = msg {
                        match msg? {
                            WsMessage::Text(text) => {
                                println!("收到WebSocket文本消息: {}", text);
                                if let Ok(ws_msg) = WebSocketMessage::from_json(&text) {
                                    println!("成功解析WebSocket消息: {:?}", ws_msg);
                                    match ws_msg {
                                        WebSocketMessage::JoinRoom {
                                            room_id,
                                            user_id: uid,
                                        } => {
                                            // 验证用户身份
                                            if let Some(u) = self.user_repo.find_by_id(&uid).await? {
                                                user_id = Some(uid.clone());
                                                current_room = Some(room_id.clone());

                                                // 获取房间的广播接收器
                                                room_receiver = Some(self.get_room_receiver(&room_id));

                                                // 加入房间
                                                self.session_manager
                                                    .add_user_to_room(&uid, &room_id)
                                                    .await?;

                                                // 发送成功消息
                                                let success_msg = WebSocketMessage::Success {
                                                    message: format!("Joined room {}", room_id),
                                                };
                                                ws_sender
                                                    .send(WsMessage::Text(success_msg.to_json()?))
                                                    .await?;

                                                // 通知其他用户
                                                self.broadcast_to_room(
                                                    &room_id,
                                                    &WebSocketMessage::UserOnline {
                                                        user_id: uid.clone(),
                                                        username: u.username,
                                                    },
                                                )
                                                .await;
                                            } else {
                                                let error_msg = WebSocketMessage::Error {
                                                    message: "Invalid user".to_string(),
                                                };
                                                ws_sender
                                                    .send(WsMessage::Text(error_msg.to_json()?))
                                                    .await?;
                                            }
                                        }
                                        WebSocketMessage::LeaveRoom {
                                            room_id,
                                            user_id: uid,
                                        } => {
                                            if user_id.as_ref() == Some(&uid) {
                                                self.session_manager
                                                    .remove_user_from_room(&uid, &room_id)
                                                    .await?;

                                                // 通知其他用户
                                                if let Some(_u) = self.user_repo.find_by_id(&uid).await? {
                                                    self.broadcast_to_room(
                                                        &room_id,
                                                        &WebSocketMessage::UserOffline { user_id: uid },
                                                    )
                                                    .await;
                                                }

                                                current_room = None;
                                                room_receiver = None;
                                            }
                                        }
                                        WebSocketMessage::ChatMessage {
                                            room_id,
                                            user_id: uid,
                                            content,
                                            message_type,
                                            username,
                                        } => {
                                            println!(
                                                "收到聊天消息: room_id={}, user_id={}, content={}",
                                                room_id, uid, content
                                            );
                                            // 简化用户验证 - 直接处理消息
                                            if let Some(_u) = self.user_repo.find_by_id(&uid).await? {
                                                println!("找到用户: {}", username);
                                                let msg_type = match message_type.as_str() {
                                                    "image" => MessageType::Image,
                                                    "file" => MessageType::File,
                                                    _ => MessageType::Text,
                                                };

                                                let message = Message::new(
                                                    uid.clone(),
                                                    username.clone(),
                                                    content.clone(),
                                                    room_id.clone(),
                                                    msg_type,
                                                );

                                                match self.message_repo.create(message).await {
                                                    Ok(_) => println!("消息已保存到数据库"),
                                                    Err(e) => {
                                                        println!("保存消息到数据库失败: {}", e);
                                                        continue;
                                                    }
                                                }

                                                // 广播消息到房间
                                                let broadcast_msg = WebSocketMessage::ChatMessage {
                                                    room_id: room_id.clone(),
                                                    user_id: uid,
                                                    username,
                                                    content,
                                                    message_type,
                                                };
                                                println!("准备广播消息到房间: {}", room_id);
                                                self.broadcast_to_room(&room_id, &broadcast_msg).await;
                                                println!("消息已广播");
                                            } else {
                                                // 用户不存在，发送错误消息
                                                let error_msg = WebSocketMessage::Error {
                                                    message: "User not found".to_string(),
                                                };
                                                ws_sender
                                                    .send(WsMessage::Text(error_msg.to_json()?))
                                                    .await?;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            WsMessage::Close(_) => {
                                // 用户断开连接
                                if let (Some(uid), Some(room_id)) = (&user_id, &current_room) {
                                    self.session_manager
                                        .remove_user_from_room(uid, room_id)
                                        .await?;
                                    self.session_manager.set_user_offline(uid).await?;

                                    // 通知其他用户
                                    self.broadcast_to_room(
                                        room_id,
                                        &WebSocketMessage::UserOffline {
                                            user_id: uid.clone(),
                                        },
                                    )
                                    .await;
                                }
                                break;
                            }
                            _ => {}
                        }
                    } else {
                        // 客户端断开连接
                        break;
                    }
                }
                // 处理从广播通道接收的消息
                broadcast_msg = async {
                    if let Some(ref mut receiver) = room_receiver {
                        receiver.recv().await
                    } else {
                        // 如果没有房间接收器，等待一个永远不会到来的消息
                        std::future::pending().await
                    }
                } => {
                    match broadcast_msg {
                        Ok(msg) => {
                            // 将广播消息发送给客户端
                            if let Ok(json) = msg.to_json() {
                                let _ = ws_sender.send(WsMessage::Text(json)).await;
                            }
                        }
                        Err(_) => {
                            // 广播通道关闭，重新获取接收器
                            if let Some(room_id) = &current_room {
                                room_receiver = Some(self.get_room_receiver(room_id));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn broadcast_to_room(&self, room_id: &str, message: &WebSocketMessage) {
        let mut senders = self.room_senders.lock().await;
        let sender = senders
            .entry(room_id.to_string())
            .or_insert_with(|| broadcast::channel(1000).0);

        let _ = sender.send(message.clone());
    }

    pub fn get_room_receiver(&self, room_id: &str) -> broadcast::Receiver<WebSocketMessage> {
        let mut senders = futures::executor::block_on(self.room_senders.lock());
        let sender = senders
            .entry(room_id.to_string())
            .or_insert_with(|| broadcast::channel(1000).0);

        sender.subscribe()
    }
}
