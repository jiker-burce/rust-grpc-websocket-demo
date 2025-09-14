use super::{MessageContext, MessageEventHandler, MessageResult};
use crate::chat::JoinRoomRequest;
use crate::database::UserRepository;
use crate::redis::SessionManager;
use crate::websocket::{UserTracker, WebSocketMessage};
use prost::Message;
use std::sync::Arc;

/// 加入房间消息事件处理器
pub struct JoinRoomHandler {
    user_repo: Arc<UserRepository>,
    session_manager: Arc<SessionManager>,
    user_tracker: Arc<UserTracker>,
}

impl JoinRoomHandler {
    pub fn new(
        user_repo: Arc<UserRepository>,
        session_manager: Arc<SessionManager>,
        user_tracker: Arc<UserTracker>,
    ) -> Self {
        Self {
            user_repo,
            session_manager,
            user_tracker,
        }
    }
}

#[async_trait::async_trait]
impl MessageEventHandler for JoinRoomHandler {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        if let WebSocketMessage::JoinRoom { room_id, user_id } = message {
            println!("处理加入房间请求: room_id={}, user_id={}", room_id, user_id);

            // 从数据库获取真实用户信息
            // 前端现在发送的是数据库中的真实用户ID
            let user = self
                .user_repo
                .find_by_id(&user_id)
                .await
                .map_err(|e| format!("数据库查询失败: {}", e))?
                .ok_or_else(|| format!("用户不存在: {}", user_id))?;

            let real_user_id = user.id.clone();
            let username = user.username.clone();

            // 从上下文中获取连接ID
            let connection_id = context
                .connection_id
                .clone()
                .unwrap_or_else(|| format!("conn_{}", real_user_id));

            // 检查用户是否已经连接，如果已连接则切换房间，否则连接并加入房间
            if let Some((old_room_id, new_room_id)) = self
                .user_tracker
                .user_switch_room(connection_id.clone(), room_id.clone())
                .await
            {
                // 用户切换房间
                println!(
                    "用户 {} 从房间 {} 切换到房间 {}",
                    username, old_room_id, new_room_id
                );
            } else {
                // 用户首次连接并加入房间
                self.user_tracker
                    .user_connect_and_join_room(
                        connection_id,
                        real_user_id.clone(),
                        username.clone(),
                        room_id.clone(),
                    )
                    .await;
            }

            // 获取房间的广播接收器
            let mut broadcast_handler = context.broadcast_handler.lock().await;
            let room_receiver = broadcast_handler
                .get_or_create_room_channel(&room_id)
                .subscribe();

            // 广播用户加入房间的消息
            let broadcast_message = WebSocketMessage::UserJoined {
                user_id: user_id.clone(),
                username: username.clone(),
                room_id: room_id.clone(),
            };
            broadcast_handler.broadcast_to_room(&room_id, &broadcast_message);

            // 获取房间用户列表并广播
            let room_users = self.user_tracker.get_room_users(&room_id).await;
            let users_list_message = WebSocketMessage::OnlineUsersList {
                room_id: room_id.clone(),
                users: room_users,
            };
            broadcast_handler.broadcast_to_room(&room_id, &users_list_message);

            drop(broadcast_handler);

            println!("用户 {} 成功加入房间: {}", username, room_id);

            // 返回设置用户ID和房间接收器的结果
            Ok(MessageResult::SetUserIdAndRoomReceiver(
                real_user_id,
                room_receiver,
            ))
        } else {
            Err("消息类型不匹配".into())
        }
    }

    fn supported_message_type(&self) -> &'static str {
        "join_room"
    }
}
