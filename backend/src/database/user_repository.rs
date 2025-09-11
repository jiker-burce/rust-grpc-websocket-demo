use crate::database::DbPool;
use crate::models::{CreateUser, UpdateUser, User};
use sqlx::Error;

pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    pub async fn create(&self, create_user: CreateUser) -> Result<User, Error> {
        let password_hash = bcrypt::hash(&create_user.password, bcrypt::DEFAULT_COST)
            .map_err(|e| Error::Protocol(format!("Password hashing failed: {}", e)))?;

        let user = User::new(create_user.username, create_user.email, password_hash);

        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, password_hash, avatar, is_online, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.avatar,
            user.is_online,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn update(&self, id: &str, update_user: UpdateUser) -> Result<User, Error> {
        let mut user = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::RowNotFound)?;

        if let Some(username) = update_user.username {
            user.username = username;
        }
        if let Some(avatar) = update_user.avatar {
            user.avatar = Some(avatar);
        }
        user.updated_at = chrono::Utc::now();

        sqlx::query!(
            r#"
            UPDATE users 
            SET username = ?, avatar = ?, updated_at = ?
            WHERE id = ?
            "#,
            user.username,
            user.avatar,
            user.updated_at,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn set_online_status(&self, id: &str, is_online: bool) -> Result<(), Error> {
        sqlx::query!("UPDATE users SET is_online = ? WHERE id = ?", is_online, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_online_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE is_online = 1 ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn verify_password(&self, user: &User, password: &str) -> Result<bool, Error> {
        bcrypt::verify(password, &user.password_hash)
            .map_err(|e| Error::Protocol(format!("Password verification failed: {}", e)))
    }
}
