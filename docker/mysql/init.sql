-- 创建数据库
CREATE DATABASE IF NOT EXISTS chat_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 使用数据库
USE chat_db;

-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(36) PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    avatar TEXT,
    is_online BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_email (email),
    INDEX idx_username (username),
    INDEX idx_online (is_online)
);

-- 创建房间表
CREATE TABLE IF NOT EXISTS rooms (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    is_public BOOLEAN DEFAULT TRUE,
    created_by VARCHAR(36) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_public (is_public),
    INDEX idx_created_by (created_by)
);

-- 创建消息表
CREATE TABLE IF NOT EXISTS messages (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    username VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    room_id VARCHAR(36) NOT NULL,
    message_type ENUM('text', 'image', 'file', 'system') DEFAULT 'text',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_room_id (room_id),
    INDEX idx_user_id (user_id),
    INDEX idx_created_at (created_at),
    INDEX idx_room_created (room_id, created_at)
);

-- 创建房间成员表
CREATE TABLE IF NOT EXISTS room_members (
    id VARCHAR(36) PRIMARY KEY,
    room_id VARCHAR(36) NOT NULL,
    user_id VARCHAR(36) NOT NULL,
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE KEY unique_room_user (room_id, user_id),
    INDEX idx_room_id (room_id),
    INDEX idx_user_id (user_id)
);

-- 插入默认房间
INSERT INTO rooms (id, name, description, is_public, created_by) 
VALUES ('general', 'General', 'General chat room for everyone', TRUE, 'system')
ON DUPLICATE KEY UPDATE name = name;

INSERT INTO rooms (id, name, description, is_public, created_by) 
VALUES ('tech', 'Tech Discussion', 'Technology discussion room', TRUE, 'system')
ON DUPLICATE KEY UPDATE name = name;

INSERT INTO rooms (id, name, description, is_public, created_by) 
VALUES ('random', 'Random Chat', 'Random chat room', TRUE, 'system')
ON DUPLICATE KEY UPDATE name = name;
