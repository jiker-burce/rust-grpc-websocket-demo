use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
    pub username: String,
    pub current_room: Option<String>,
}

/// 房间用户列表管理器
#[derive(Debug)]
pub struct UserTracker {
    // 连接ID -> 用户信息映射
    connection_users: Arc<RwLock<HashMap<String, UserInfo>>>,
    // 房间ID -> 用户信息列表映射
    room_users: Arc<RwLock<HashMap<String, Vec<UserInfo>>>>,
}

impl UserTracker {
    pub fn new() -> Self {
        Self {
            connection_users: Arc::new(RwLock::new(HashMap::new())),
            room_users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 用户连接并加入房间
    pub async fn user_connect_and_join_room(
        &self,
        connection_id: String,
        user_id: String,
        username: String,
        room_id: String,
    ) {
        let mut connection_users = self.connection_users.write().await;
        let mut room_users = self.room_users.write().await;

        // 创建用户信息
        let user_info = UserInfo {
            user_id: user_id.clone(),
            username: username.clone(),
            current_room: Some(room_id.clone()),
        };

        // 存储连接ID到用户信息的映射
        connection_users.insert(connection_id, user_info.clone());

        // 将用户添加到房间（先移除可能存在的重复用户）
        let room_user_list = room_users.entry(room_id.clone()).or_insert_with(Vec::new);
        // 移除可能存在的相同用户ID的用户
        room_user_list.retain(|u| u.user_id != user_id);
        // 添加新用户
        room_user_list.push(user_info);

        println!("用户 {} 连接并加入房间 {}", username, room_id);
    }

    /// 用户切换房间
    pub async fn user_switch_room(
        &self,
        connection_id: String,
        new_room_id: String,
    ) -> Option<(String, String)> {
        let mut connection_users = self.connection_users.write().await;
        let mut room_users = self.room_users.write().await;

        if let Some(user_info) = connection_users.get_mut(&connection_id) {
            let old_room_id = user_info.current_room.clone();
            let user_id = user_info.user_id.clone();
            let username = user_info.username.clone();

            // 从旧房间移除用户
            if let Some(old_room) = &old_room_id {
                if let Some(room_user_list) = room_users.get_mut(old_room) {
                    room_user_list.retain(|u| u.user_id != user_id);
                }
            }

            // 更新用户当前房间
            user_info.current_room = Some(new_room_id.clone());

            // 将用户添加到新房间（先移除可能存在的重复用户）
            let room_user_list = room_users
                .entry(new_room_id.clone())
                .or_insert_with(Vec::new);
            room_user_list.retain(|u| u.user_id != user_id);
            room_user_list.push(user_info.clone());

            println!(
                "用户 {} 从房间 {:?} 切换到房间 {}",
                username, old_room_id, new_room_id
            );
            return old_room_id.map(|old_room| (old_room, new_room_id));
        }

        None
    }

    /// 连接断开时清理用户
    pub async fn connection_disconnect(&self, connection_id: String) -> Option<(String, String)> {
        let mut connection_users = self.connection_users.write().await;
        let mut room_users = self.room_users.write().await;

        if let Some(user_info) = connection_users.remove(&connection_id) {
            let user_id = user_info.user_id.clone();
            let username = user_info.username.clone();
            let room_id = user_info.current_room.clone();

            // 从房间中移除用户
            if let Some(room) = &room_id {
                if let Some(room_user_list) = room_users.get_mut(room) {
                    room_user_list.retain(|u| u.user_id != user_id);
                }
            }

            println!("连接断开，用户 {} 已从房间 {:?} 中移除", username, room_id);
            return room_id.map(|room| (user_id, room));
        }

        None
    }

    /// 获取房间内的用户列表
    pub async fn get_room_users(&self, room_id: &str) -> Vec<UserInfo> {
        let room_users = self.room_users.read().await;
        room_users.get(room_id).cloned().unwrap_or_default()
    }

    /// 获取房间内的用户数量
    pub async fn get_room_user_count(&self, room_id: &str) -> usize {
        let room_users = self.room_users.read().await;
        room_users
            .get(room_id)
            .map(|users| users.len())
            .unwrap_or(0)
    }

    /// 根据连接ID获取用户信息
    pub async fn get_user_by_connection(&self, connection_id: &str) -> Option<UserInfo> {
        let connection_users = self.connection_users.read().await;
        connection_users.get(connection_id).cloned()
    }
}
