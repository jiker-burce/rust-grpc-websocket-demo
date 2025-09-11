use crate::database::DbPool;
use crate::models::Room;
use sqlx::Error;

pub struct RoomRepository {
    pool: DbPool,
}

impl RoomRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, room: Room) -> Result<Room, Error> {
        sqlx::query!(
            r#"
            INSERT INTO rooms (id, name, description, is_public, created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            room.id,
            room.name,
            room.description,
            room.is_public,
            room.created_by,
            room.created_at,
            room.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(room)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Room>, Error> {
        let room = sqlx::query_as!(Room, "SELECT * FROM rooms WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(room)
    }

    pub async fn get_public_rooms(&self) -> Result<Vec<Room>, Error> {
        let rooms = sqlx::query_as!(
            Room,
            "SELECT * FROM rooms WHERE is_public = 1 ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rooms)
    }

    pub async fn get_user_rooms(&self, user_id: &str) -> Result<Vec<Room>, Error> {
        let rooms = sqlx::query_as!(
            Room,
            r#"
            SELECT r.* FROM rooms r
            LEFT JOIN room_members rm ON r.id = rm.room_id
            WHERE r.is_public = 1 OR rm.user_id = ? OR r.created_by = ?
            ORDER BY r.updated_at DESC
            "#,
            user_id,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rooms)
    }
}
