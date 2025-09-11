use sqlx::{MySql, Pool};
use std::env;

pub type DbPool = Pool<MySql>;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@localhost:3306/chat_db".to_string());

    let pool = sqlx::MySqlPool::connect(&database_url).await?;

    // 运行数据库迁移
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn init_database() -> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@localhost:3306/chat_db".to_string());

    // 创建数据库（如果不存在）
    let database_name = "chat_db";
    let create_db_url = database_url.replace(&format!("/{}", database_name), "");

    let pool = sqlx::MySqlPool::connect(&create_db_url).await?;
    sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS {}", database_name))
        .execute(&pool)
        .await?;
    pool.close().await;

    Ok(())
}
