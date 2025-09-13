use crate::websocket::WebSocketMessage;
use tokio::sync::broadcast;

/// 连接状态管理，跟踪单个WebSocket连接的状态
pub struct ConnectionState {
    pub user_id: Option<String>,
    pub current_room: Option<String>,
    pub room_receiver: Option<broadcast::Receiver<WebSocketMessage>>,
}

impl ConnectionState {
    pub fn new() -> Self {
        Self {
            user_id: None,
            current_room: None,
            room_receiver: None,
        }
    }

    /// 设置用户ID
    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = Some(user_id);
    }

    /// 设置当前房间
    pub fn set_current_room(&mut self, room_id: String) {
        self.current_room = Some(room_id);
    }

    /// 清除当前房间
    pub fn clear_current_room(&mut self) {
        self.current_room = None;
        self.room_receiver = None;
    }

    /// 设置房间接收器
    pub fn set_room_receiver(&mut self, receiver: broadcast::Receiver<WebSocketMessage>) {
        self.room_receiver = Some(receiver);
    }

    /// 获取当前用户ID的引用
    pub fn get_user_id(&self) -> &Option<String> {
        &self.user_id
    }

    /// 获取当前房间ID的引用
    pub fn get_current_room(&self) -> &Option<String> {
        &self.current_room
    }
}
