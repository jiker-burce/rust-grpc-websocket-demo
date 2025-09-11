use redis::{AsyncCommands, Client, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub username: String,
    pub room_id: Option<String>,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Clone)]
pub struct SessionManager {
    client: Client,
}

impl SessionManager {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_session(&self, user_id: String, username: String) -> RedisResult<String> {
        let mut conn = self.client.get_async_connection().await?;

        let session_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let expires_at = now + 24 * 60 * 60; // 24小时过期

        let session = Session {
            user_id: user_id.clone(),
            username,
            room_id: None,
            created_at: now,
            expires_at,
        };

        let session_key = format!("session:{}", session_id);
        let user_session_key = format!("user_session:{}", user_id);

        // 存储会话信息
        let session_json = serde_json::to_string(&session).map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "JSON serialization failed",
                e.to_string(),
            ))
        })?;
        redis::pipe()
            .set(&session_key, &session_json)
            .expire(&session_key, 24 * 60 * 60)
            .set(&user_session_key, &session_id)
            .expire(&user_session_key, 24 * 60 * 60)
            .query_async::<_, ()>(&mut conn)
            .await?;

        Ok(session_id)
    }

    pub async fn get_session(&self, session_id: &str) -> RedisResult<Option<Session>> {
        let mut conn = self.client.get_async_connection().await?;
        let session_key = format!("session:{}", session_id);

        let session_data: Option<String> = conn.get(&session_key).await?;

        if let Some(data) = session_data {
            let session: Session = serde_json::from_str(&data).map_err(|e| {
                redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "JSON deserialization failed",
                    e.to_string(),
                ))
            })?;
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    pub async fn update_session_room(
        &self,
        session_id: &str,
        room_id: Option<String>,
    ) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        let session_key = format!("session:{}", session_id);

        if let Some(mut session) = self.get_session(session_id).await? {
            session.room_id = room_id;
            let session_json = serde_json::to_string(&session).map_err(|e| {
                redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "JSON serialization failed",
                    e.to_string(),
                ))
            })?;
            conn.set::<_, _, ()>(&session_key, &session_json).await?;
        }

        Ok(())
    }

    pub async fn delete_session(&self, session_id: &str) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        let session_key = format!("session:{}", session_id);

        if let Some(session) = self.get_session(session_id).await? {
            let user_session_key = format!("user_session:{}", session.user_id);
            redis::pipe()
                .del(&session_key)
                .del(&user_session_key)
                .query_async::<_, ()>(&mut conn)
                .await?;
        }

        Ok(())
    }

    pub async fn add_user_to_room(&self, user_id: &str, room_id: &str) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        let room_users_key = format!("room_users:{}", room_id);

        conn.sadd::<_, _, ()>(&room_users_key, user_id).await?;
        conn.expire::<_, ()>(&room_users_key, 60 * 60).await?; // 1小时过期

        Ok(())
    }

    pub async fn remove_user_from_room(&self, user_id: &str, room_id: &str) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        let room_users_key = format!("room_users:{}", room_id);

        conn.srem::<_, _, ()>(&room_users_key, user_id).await?;

        Ok(())
    }

    pub async fn get_room_users(&self, room_id: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.client.get_async_connection().await?;
        let room_users_key = format!("room_users:{}", room_id);

        let users: Vec<String> = conn.smembers(&room_users_key).await?;
        Ok(users)
    }

    pub async fn set_user_online(&self, user_id: &str) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        let online_users_key = "online_users";

        conn.sadd::<_, _, ()>(online_users_key, user_id).await?;
        conn.expire::<_, ()>(online_users_key, 60 * 60).await?; // 1小时过期

        Ok(())
    }

    pub async fn set_user_offline(&self, user_id: &str) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        let online_users_key = "online_users";

        conn.srem::<_, _, ()>(online_users_key, user_id).await?;

        Ok(())
    }

    pub async fn get_online_users(&self) -> RedisResult<Vec<String>> {
        let mut conn = self.client.get_async_connection().await?;
        let online_users_key = "online_users";

        let users: Vec<String> = conn.smembers(online_users_key).await?;
        Ok(users)
    }
}
