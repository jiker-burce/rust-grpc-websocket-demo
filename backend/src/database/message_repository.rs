use crate::database::DbPool;
use crate::models::{Message, MessageType};
use sqlx::Error;

pub struct MessageRepository {
    pool: DbPool,
}

impl MessageRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, message: Message) -> Result<Message, Error> {
        sqlx::query!(
            r#"
            INSERT INTO messages (id, user_id, username, content, room_id, message_type, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            message.id,
            message.user_id,
            message.username,
            message.content,
            message.room_id,
            message.message_type.to_string(),
            message.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(message)
    }

    pub async fn get_messages_by_room(
        &self,
        room_id: &str,
        limit: i32,
        before_timestamp: Option<i64>,
    ) -> Result<Vec<Message>, Error> {
        println!(
            "数据库查询 - 房间ID: {}, 限制: {}, before_timestamp: {:?}",
            room_id, limit, before_timestamp
        );

        let messages = if let Some(timestamp) = before_timestamp {
            println!("使用 before_timestamp 查询，时间戳: {}", timestamp);
            sqlx::query_as!(
                Message,
                r#"
                SELECT * FROM messages 
                WHERE room_id = ? AND created_at < FROM_UNIXTIME(?)
                ORDER BY created_at ASC 
                LIMIT ?
                "#,
                room_id,
                timestamp,
                limit
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            println!("不使用 before_timestamp 查询，获取所有消息");
            sqlx::query_as!(
                Message,
                r#"
                SELECT * FROM messages 
                WHERE room_id = ? 
                ORDER BY created_at ASC 
                LIMIT ?
                "#,
                room_id,
                limit
            )
            .fetch_all(&self.pool)
            .await?
        };

        println!("数据库返回 {} 条消息", messages.len());
        Ok(messages)
    }

    pub async fn get_recent_messages(
        &self,
        room_id: &str,
        limit: i32,
    ) -> Result<Vec<Message>, Error> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT * FROM messages 
            WHERE room_id = ? 
            ORDER BY created_at ASC 
            LIMIT ?
            "#,
            room_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(messages)
    }
}
