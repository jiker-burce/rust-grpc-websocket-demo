# 实时聊天系统 (Real-time Chat System)

一个基于 Rust + Vue.js 的综合实时聊天系统，集成了 gRPC、WebSocket、MySQL、Redis 等技术栈。

## 🚀 项目特色

- **高性能**: 基于 Rust 的异步后端，支持高并发
- **实时通信**: WebSocket 实现即时消息推送
- **现代架构**: gRPC + HTTP API 双重接口
- **响应式设计**: Vue 3 + Element Plus 现代化界面
- **完整功能**: 用户管理、实时聊天、在线状态、消息历史

## 📁 项目架构

```
grpc_websocket/
├── backend/                 # Rust 后端服务
│   ├── src/
│   │   ├── main.rs         # 主程序入口
│   │   ├── grpc/           # gRPC 服务实现
│   │   ├── http/           # HTTP API 接口
│   │   ├── websocket/      # WebSocket 处理
│   │   │   ├── old_websocket/    # 旧版WebSocket实现（已废弃）
│   │   │   └── new_websocket/    # 新版WebSocket实现（当前使用）
│   │   │       ├── handler.rs           # WebSocket连接处理器
│   │   │       ├── command_processor.rs # 命令处理器
│   │   │       ├── event_handler_factory.rs # 事件处理器工厂
│   │   │       └── event_handlers/      # 事件处理器集合
│   │   │           ├── message_handler.rs      # 消息处理器接口
│   │   │           ├── send_message_handler.rs # 发送消息处理器
│   │   │           ├── join_room_handler.rs    # 加入房间处理器
│   │   │           ├── leave_room_handler.rs   # 离开房间处理器
│   │   │           └── get_messages_handler.rs # 获取消息处理器
│   │   ├── database/       # 数据库操作
│   │   ├── redis/          # Redis 缓存
│   │   └── models/         # 数据模型
│   ├── proto/              # Protocol Buffers 定义
│   ├── migrations/         # 数据库迁移
│   └── Cargo.toml
├── frontend/               # Vue.js 前端
│   ├── src/
│   │   ├── components/     # Vue 组件
│   │   ├── views/          # 页面视图
│   │   ├── stores/         # Pinia 状态管理
│   │   ├── services/       # API 服务
│   │   └── router/         # 路由配置
│   ├── package.json
│   └── vite.config.js
├── docker/                 # Docker 配置
│   ├── docker-compose.yml
│   └── mysql/
└── scripts/               # 部署脚本
    ├── start.sh           # 生产环境启动
    ├── dev.sh             # 开发环境启动
    ├── stop.sh            # 停止服务
    └── test.sh            # 系统测试
```

### 🔄 架构演进历程

#### 旧版架构 (old_websocket) - 已废弃

**设计特点：**
- 简单的消息处理模式
- 直接在WebSocket处理器中处理所有逻辑
- 缺乏模块化和扩展性
- 用户状态管理不够完善

**主要问题：**
```rust
// 旧版架构 - 所有逻辑集中在一个处理器中
impl WebSocketHandler {
    async fn handle_websocket_message(&self, msg: WebSocketMessage) {
        match msg {
            WebSocketMessage::ChatMessage { .. } => {
                // 直接在这里处理聊天消息
                // 缺乏模块化，难以维护
            }
            WebSocketMessage::JoinRoom { .. } => {
                // 直接在这里处理加入房间
                // 逻辑耦合严重
            }
            // ... 其他消息类型
        }
    }
}
```

**存在的问题：**
1. **代码耦合严重**: 所有消息处理逻辑集中在一个方法中
2. **难以扩展**: 添加新的消息类型需要修改核心处理器
3. **测试困难**: 无法单独测试特定的消息处理逻辑
4. **用户状态管理不完善**: 缺乏有效的用户跟踪机制
5. **消息过滤机制缺失**: 无法过滤自己发送的消息

#### 新版架构 (new_websocket) - 当前使用

**设计特点：**
- 事件驱动的模块化架构
- 职责分离的处理器模式
- 完善的用户状态管理
- 智能的消息过滤机制

**核心改进：**

1. **事件驱动架构**
```rust
// 新版架构 - 事件处理器模式
pub trait MessageEventHandler: Send + Sync {
    async fn handle(&self, message: WebSocketMessage, context: &MessageContext) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>>;
    fn supported_message_type(&self) -> &'static str;
}

// 专门的发送消息处理器
impl MessageEventHandler for SendMessageHandler {
    async fn handle(&self, message: WebSocketMessage, context: &MessageContext) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
        // 专门处理发送消息的逻辑
    }
}
```

2. **完善的用户状态管理**
```rust
// UserTracker - 内存级用户管理
pub struct UserTracker {
    connection_users: Arc<RwLock<HashMap<String, UserInfo>>>,  // 连接ID -> 用户信息
    room_users: Arc<RwLock<HashMap<String, Vec<UserInfo>>>>,   // 房间ID -> 用户列表
}

impl UserTracker {
    // 高效的连接ID到用户信息映射
    pub async fn get_user_by_connection(&self, connection_id: &str) -> Option<UserInfo>;
    
    // 用户连接并加入房间
    pub async fn user_connect_and_join_room(&self, connection_id: String, user_id: String, username: String, room_id: String);
}
```

3. **智能消息过滤机制**
```rust
// 消息过滤逻辑 - 避免自己发送的消息重复显示
pub fn should_send_to_client(&self, message: &WebSocketMessage, current_user_id: &Option<String>) -> bool {
    match message {
        WebSocketMessage::NewMessage { message } => {
            let msg_user_id = &message.user_id;
            if let Some(current_user) = current_user_id {
                msg_user_id != current_user  // 过滤自己发送的消息
            } else {
                true
            }
        }
        _ => true
    }
}
```

4. **模块化的处理器系统**
```rust
// 事件处理器工厂 - 统一管理所有处理器
pub struct EventHandlerFactory {
    handlers: HashMap<String, MessageEventHandlerEnum>,
}

impl EventHandlerFactory {
    pub fn new(/* 依赖注入 */) -> Self {
        let mut handlers = HashMap::new();
        
        // 注册各种处理器
        handlers.insert("send_message".to_string(), MessageEventHandlerEnum::SendMessage(/* ... */));
        handlers.insert("join_room".to_string(), MessageEventHandlerEnum::JoinRoom(/* ... */));
        handlers.insert("leave_room".to_string(), MessageEventHandlerEnum::LeaveRoom(/* ... */));
        
        Self { handlers }
    }
}
```

#### 架构对比总结

| 特性 | 旧版架构 (old_websocket) | 新版架构 (new_websocket) |
|------|-------------------------|-------------------------|
| **代码组织** | 集中式处理，逻辑耦合 | 模块化处理，职责分离 |
| **扩展性** | 难以扩展新功能 | 易于添加新的消息类型 |
| **测试性** | 难以单独测试 | 可独立测试各个处理器 |
| **用户管理** | 基础的用户状态管理 | 完善的UserTracker系统 |
| **消息过滤** | 无过滤机制 | 智能消息过滤 |
| **性能优化** | 频繁数据库查询 | 内存级用户管理 |
| **维护性** | 难以维护 | 易于维护和调试 |
| **类型安全** | 基础类型检查 | 强类型接口和错误处理 |

#### 迁移原因

1. **可维护性提升**: 模块化架构使得代码更易于理解和维护
2. **性能优化**: 内存级用户管理减少了数据库查询次数
3. **用户体验改善**: 智能消息过滤避免了重复消息显示
4. **扩展性增强**: 事件驱动架构便于添加新功能
5. **代码质量**: 更好的错误处理和类型安全

#### 向后兼容性

- 保留了 `old_websocket` 目录作为参考
- 新版架构完全兼容现有的前端接口
- 数据库结构保持不变
- API接口保持一致性

## 🏗️ 系统架构设计

### 架构概述

本项目采用WebSocket和gRPC混合架构，充分发挥两种技术的优势：

- **WebSocket**: 专注实时通信和双向交互
- **gRPC**: 专注请求响应操作和数据持久化

### 🔗 gRPC接口一致性保证

#### 代码生成流程

**1. 后端代码生成 (Rust)**
```rust
// build.rs - 编译时自动生成
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/chat.proto")?;
    Ok(())
}
```
- 使用 `tonic-build` 从 `backend/proto/chat.proto` 生成Rust代码
- 生成到 `backend/src/grpc/` 目录
- 编译时自动执行，确保类型安全

**2. 前端代码生成 (JavaScript/TypeScript)**
```bash
# 使用 protoc 命令行工具生成
protoc --js_out=import_style=commonjs,binary:src/services/grpc-generated \
       --grpc-web_out=import_style=typescript,mode=grpcwebtext:src/services/grpc-generated \
       proto/chat.proto
```
- 从 `backend/proto/chat.proto` 生成前端代码
- 生成到 `frontend/src/services/grpc-generated/` 目录
- 包含客户端代码和类型定义

**3. 文件复制位置**
```
backend/proto/chat.proto  ←── 唯一的接口定义源
         ↓
    ┌─────────┬─────────┐
    ↓         ↓         ↓
backend/src/grpc/    frontend/src/services/grpc-generated/
(Rust代码)          (JS/TS代码)
```

#### 一致性保证

通过 **Protocol Buffers** 作为单一数据源，前后端接口数据类型完全一致：

- **类型安全**: 编译时检查，避免字段名或类型错误
- **自动同步**: 修改proto文件后重新生成，前后端接口自动同步
- **开发效率**: 减少手工维护接口的工作量，专注业务逻辑开发

### 设计原则

#### 1. 职责分离原则
- **WebSocket**: 实时通信、事件广播、状态同步
- **gRPC**: 数据查询、业务操作、持久化存储
- **HTTP**: 静态资源、健康检查、认证接口

#### 2. 性能优化原则
- **内存级用户管理**: 使用 `UserTracker` 进行在线用户管理，避免频繁数据库查询
- **高效消息过滤**: 实现消息发送者过滤，避免重复广播
- **智能时间戳处理**: 自动识别秒/毫秒时间戳，提供友好的时间显示

#### 3. 可扩展性原则
- **事件驱动架构**: 使用 `EventHandler` 模式，易于添加新的消息类型
- **模块化设计**: 前后端分离，服务独立部署
- **类型安全**: 使用强类型接口，减少运行时错误

### 前后端交互架构

#### 前端架构
```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend (Vue.js)                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │   WebSocket     │  │   gRPC Client   │  │ HTTP Client  │ │
│  │   Store         │  │   Service       │  │   Service    │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
│           │                     │                    │      │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │  Chat Hybrid    │  │  gRPC Client    │  │   API        │ │
│  │  Store          │  │  Service        │  │   Service    │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
           │                     │                    │
           │ WebSocket           │ gRPC               │ HTTP
           │ (实时通信)           │ (数据查询)          │ (静态资源)
           ▼                     ▼                    ▼
┌─────────────────────────────────────────────────────────────┐
│                        Backend (Rust)                       │
└─────────────────────────────────────────────────────────────┘
```

#### 后端架构
```
┌─────────────────────────────────────────────────────────────┐
│                        Backend Services                     │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │ WebSocket       │  │ gRPC Server     │  │ HTTP Server  │ │
│  │ Server          │  │ (Port 50051)    │  │ (Port 3001)  │ │
│  │ (Port 8301)     │  │                 │  │              │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
│           │                     │                    │      │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │ Event Handler   │  │ Chat Service    │  │ Static       │ │
│  │ System          │  │ User Service    │  │ Resources    │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
│           │                     │                    │      │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │ User Tracker    │  │ Database        │  │ Redis        │ │
│  │ (内存管理)        │  │ (MySQL)         │  │ (Session)    │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 🛠 技术栈

### 后端 (Rust)
- **gRPC**: 高性能 RPC 通信
- **WebSocket**: 实时消息推送
- **MySQL**: 用户信息、聊天记录存储
- **Redis**: 在线状态、会话缓存
- **Tonic**: gRPC 框架
- **Tokio**: 异步运行时
- **SQLx**: 类型安全的数据库操作
- **Warp**: HTTP 服务器
- **JWT**: 用户认证

### 前端 (Vue.js)
- **Vue 3**: 现代前端框架
- **Vite**: 快速构建工具
- **Element Plus**: 企业级 UI 组件库
- **Pinia**: 状态管理
- **Vue Router**: 路由管理
- **Axios**: HTTP 请求库
- **WebSocket**: 实时通信

## 🔧 核心实现架构

### WebSocket 架构详解

#### 连接管理
```rust
// 连接状态管理
pub struct ConnectionState {
    pub user_id: Option<String>,           // 当前用户ID
    pub current_room: Option<String>,      // 当前房间
    pub room_receiver: Option<broadcast::Receiver<WebSocketMessage>>, // 房间消息接收器
}
```

#### 事件处理器系统
```rust
// 事件处理器接口
pub trait MessageEventHandler: Send + Sync {
    async fn handle(&self, message: WebSocketMessage, context: &MessageContext) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>>;
    fn supported_message_type(&self) -> &'static str;
}

// 支持的消息类型
pub enum MessageResult {
    NoOp,
    SetUserId(String),
    SetCurrentRoom(String),
    SetUserIdAndRoomReceiver(String, broadcast::Receiver<WebSocketMessage>),
    SendResponse(WebSocketMessage),
}
```

#### 消息过滤机制
```rust
// 消息过滤逻辑
pub fn should_send_to_client(&self, message: &WebSocketMessage, current_user_id: &Option<String>) -> bool {
    match message {
        WebSocketMessage::NewMessage { message } => {
            // 新消息：只有不是当前用户发送的才转发
            let msg_user_id = &message.user_id;
            if let Some(current_user) = current_user_id {
                msg_user_id != current_user  // 过滤自己发送的消息
            } else {
                true
            }
        }
        _ => true  // 其他类型消息直接发送
    }
}
```

### gRPC 架构详解

#### 服务定义
```protobuf
service ChatService {
    rpc SendMessage(SendMessageRequest) returns (SendMessageResponse);
    rpc GetMessages(GetMessagesRequest) returns (GetMessagesResponse);
    rpc GetOnlineUsers(GetOnlineUsersRequest) returns (GetOnlineUsersResponse);
}

service UserService {
    rpc CreateUser(CreateUserRequest) returns (CreateUserResponse);
    rpc GetUser(GetUserRequest) returns (GetUserResponse);
    rpc UpdateUser(UpdateUserRequest) returns (UpdateUserResponse);
}
```

#### 数据流处理
```rust
// 消息存储流程
impl ChatServiceImpl {
    async fn send_message(&self, request: Request<SendMessageRequest>) -> Result<Response<SendMessageResponse>, Status> {
        // 1. 验证用户身份
        // 2. 存储消息到数据库
        // 3. 返回响应
    }
    
    async fn get_messages(&self, request: Request<GetMessagesRequest>) -> Result<Response<GetMessagesResponse>, Status> {
        // 1. 解析请求参数
        // 2. 查询数据库
        // 3. 按时间升序返回消息
    }
}
```

## ⚡ 核心实现关键点

### 1. 用户状态管理
```rust
// UserTracker - 内存级用户管理
pub struct UserTracker {
    connection_users: Arc<RwLock<HashMap<String, UserInfo>>>,  // 连接ID -> 用户信息
    room_users: Arc<RwLock<HashMap<String, Vec<UserInfo>>>>,   // 房间ID -> 用户列表
}

impl UserTracker {
    // 用户连接并加入房间
    pub async fn user_connect_and_join_room(&self, connection_id: String, user_id: String, username: String, room_id: String);
    
    // 根据连接ID获取用户信息（高效查询）
    pub async fn get_user_by_connection(&self, connection_id: &str) -> Option<UserInfo>;
}
```

### 2. 消息广播系统
```rust
// BroadcastHandler - 房间广播管理
pub struct BroadcastHandler {
    room_channels: HashMap<String, broadcast::Sender<WebSocketMessage>>,
}

impl BroadcastHandler {
    // 广播消息到指定房间
    pub fn broadcast_to_room(&self, room_id: &str, message: &WebSocketMessage);
    
    // 获取房间的广播接收器
    pub fn get_room_receiver(&self, room_id: &str) -> Option<broadcast::Receiver<WebSocketMessage>>;
}
```

### 3. 时间戳处理优化
```javascript
// 前端智能时间戳格式化
const formatTimestamp = (timestamp) => {
    // 自动识别秒/毫秒时间戳
    const timestampMs = timestamp < 10000000000 ? timestamp * 1000 : timestamp;
    const messageTime = dayjs(timestampMs);
    const now = dayjs();
    
    // 根据时间差显示不同格式
    if (messageTime.isSame(now, 'day')) {
        return messageTime.format('HH:mm:ss');           // 今天：只显示时间
    } else if (messageTime.isSame(now.subtract(1, 'day'), 'day')) {
        return `昨天 ${messageTime.format('HH:mm:ss')}`;  // 昨天：显示"昨天+时间"
    } else if (messageTime.isSame(now, 'year')) {
        return messageTime.format('MM-DD HH:mm:ss');     // 今年：显示月日+时间
    } else {
        return messageTime.format('YYYY-MM-DD HH:mm:ss'); // 其他：显示完整日期
    }
};
```

## 📊 消息流程详解

### 发送消息流程
```
1. 前端发送消息
   ↓
2. WebSocket接收消息 (SendMessageHandler)
   ↓
3. 从UserTracker获取用户信息（内存查询，高效）
   ↓
4. 创建ChatMessage对象
   ↓
5. 存储消息到数据库 (gRPC调用)
   ↓
6. 广播消息到房间 (BroadcastHandler)
   ↓
7. 消息过滤 (should_send_to_client)
   ↓
8. 发送给房间内其他用户（排除发送者）
```

### 获取历史消息流程
```
1. 前端请求历史消息
   ↓
2. gRPC调用GetMessages
   ↓
3. 数据库查询（按时间升序）
   ↓
4. 返回消息列表
   ↓
5. 前端格式化时间戳
   ↓
6. 显示在聊天界面
```

### 用户加入房间流程
```
1. 前端发送JoinRoom请求
   ↓
2. WebSocket接收 (JoinRoomHandler)
   ↓
3. 验证用户身份（数据库查询）
   ↓
4. 更新UserTracker（内存管理）
   ↓
5. 设置ConnectionState（用户ID + 房间接收器）
   ↓
6. 广播用户加入消息
   ↓
7. 获取并广播在线用户列表
```

## ✨ 功能特性

- ✅ **用户系统**: 注册、登录、个人资料管理
- ✅ **实时聊天**: 即时消息发送和接收
- ✅ **多房间支持**: 支持多个聊天房间
- ✅ **在线状态**: 实时显示在线用户
- ✅ **消息历史**: 聊天记录持久化存储
- ✅ **响应式设计**: 适配各种设备尺寸
- ✅ **安全认证**: JWT Token 认证机制
- ✅ **会话管理**: Redis 缓存用户会话
- ✅ **智能过滤**: 避免自己发送的消息重复显示
- ✅ **友好时间**: 智能时间戳格式化显示
- ✅ **高效查询**: 内存级用户管理，减少数据库查询

## 🚀 快速开始

### 环境要求
- **Rust**: 1.70+
- **Node.js**: 18+
- **Docker**: 20.0+
- **Docker Compose**: 2.0+

### 方式一：Docker 一键部署（推荐）

```bash
# 克隆项目
git clone <repository-url>
cd grpc_websocket

# 一键启动所有服务
./scripts/start.sh
```

### 方式二：开发环境

```bash
# 启动开发环境
./scripts/dev.sh
```

### 方式三：手动启动

1. **启动数据库服务**
```bash
cd docker
docker-compose up -d mysql redis
```

2. **启动后端服务**
```bash
cd backend
cargo run
```

3. **启动前端服务**
```bash
cd frontend
npm install
npm run dev
```

## 📱 访问地址

- **前端界面**: http://localhost:3002
- **后端 API**: http://localhost:3001
- **健康检查**: http://localhost:3001/health
- **gRPC 服务**: localhost:50051
- **WebSocket**: ws://localhost:8301

## 🧪 系统测试

```bash
# 运行系统测试
./scripts/test.sh
```

## 📊 数据库设计

### 用户表 (users)

- `id`: 用户唯一标识
- `username`: 用户名
- `email`: 邮箱地址
- `password_hash`: 密码哈希
- `avatar`: 头像URL
- `is_online`: 在线状态
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 消息表 (messages)

- `id`: 消息唯一标识
- `user_id`: 发送用户ID
- `username`: 发送用户名
- `content`: 消息内容
- `room_id`: 房间ID
- `message_type`: 消息类型
- `created_at`: 发送时间

### 房间表 (rooms)

- `id`: 房间唯一标识
- `name`: 房间名称
- `description`: 房间描述
- `is_public`: 是否公开
- `created_by`: 创建者ID
- `created_at`: 创建时间
- `updated_at`: 更新时间

## 🔧 API 接口

### 用户相关

- `POST /users/register` - 用户注册
- `POST /users/login` - 用户登录

### 聊天相关

- `POST /chat/messages` - 发送消息
- `GET /chat/rooms/{room_id}/messages` - 获取消息历史
- `GET /chat/rooms/{room_id}/users` - 获取在线用户

## 🐳 Docker 部署

### 生产环境部署

```bash
# 构建并启动所有服务
docker-compose -f docker/docker-compose.yml up --build -d

# 查看服务状态
docker-compose -f docker/docker-compose.yml ps

# 查看日志
docker-compose -f docker/docker-compose.yml logs -f

# 停止服务
docker-compose -f docker/docker-compose.yml down
```

## 🔒 安全配置

### 环境变量配置

创建 `backend/.env` 文件：

```env
DATABASE_URL=mysql://chat_user:chat_password@localhost:3306/chat_db
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-jwt-key-change-in-production
```

### 生产环境注意事项

1. **更改默认密码**: 修改数据库和Redis的默认密码
2. **JWT密钥**: 使用强随机密钥替换默认JWT密钥
3. **HTTPS**: 生产环境建议使用HTTPS
4. **防火墙**: 配置适当的防火墙规则
5. **备份**: 定期备份数据库数据

## 📈 性能优化

### 核心优化策略

1. **内存级用户管理**: 使用 `UserTracker` 在内存中管理在线用户，避免频繁的数据库查询
2. **消息过滤优化**: 在广播层面过滤自己发送的消息，减少不必要的网络传输
3. **时间戳处理优化**: 自动识别时间戳格式（秒/毫秒），智能显示时间格式
4. **数据库查询优化**: 使用索引优化查询性能，按时间升序排列消息

### 其他优化

- **连接池**: 数据库连接池优化
- **缓存策略**: Redis缓存热点数据
- **异步处理**: 全异步架构提升并发性能
- **消息队列**: 可扩展消息队列处理
- **负载均衡**: 支持水平扩展

## 🛡️ 错误处理策略

### WebSocket错误处理
```rust
// 连接错误处理
match self.handle_websocket_message(msg?, &mut ws_sender, &mut connection_state, &mut message_context).await {
    Ok(_) => {},
    Err(e) => {
        println!("处理WebSocket消息失败: {}", e);
        break; // 断开连接
    }
}
```

### gRPC错误处理
```rust
// 服务错误处理
async fn send_message(&self, request: Request<SendMessageRequest>) -> Result<Response<SendMessageResponse>, Status> {
    match self.process_send_message(request).await {
        Ok(response) => Ok(Response::new(response)),
        Err(e) => {
            println!("发送消息失败: {}", e);
            Err(Status::internal("发送消息失败"))
        }
    }
}
```

## 📊 监控和日志

### 关键指标监控
- WebSocket连接数
- 消息发送频率
- gRPC请求延迟
- 数据库查询性能
- 内存使用情况

### 日志策略
```rust
// 结构化日志
println!("用户 {} 连接并加入房间 {}", username, room_id);
println!("成功广播消息到房间 {} (排除发送者 {}): {:?}", room_id, sender_user_id, message);
println!("数据库返回 {} 条消息", messages.len());
```

## 🚀 扩展性考虑

### 水平扩展
- WebSocket服务可以部署多个实例
- 使用Redis进行会话共享
- 数据库读写分离

### 功能扩展
- 支持文件上传
- 支持消息加密
- 支持多语言
- 支持消息搜索

## 🔒 安全考虑

### 认证和授权
- JWT Token认证
- 用户权限验证
- 房间访问控制

### 数据安全
- 消息内容验证
- SQL注入防护
- XSS攻击防护

## 🤝 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

感谢以下开源项目：

- [Rust](https://www.rust-lang.org/)
- [Vue.js](https://vuejs.org/)
- [Element Plus](https://element-plus.org/)
- [Tonic](https://github.com/hyperium/tonic)
- [SQLx](https://github.com/launchbadge/sqlx)

## 工作机会

- 有合适的rust岗位希望大家可以帮我推荐，以后会陆续推出各种demo项目共大家研究学习，共同进步。
- 邮箱：[storefee@163.com](storefee@163.com)