# WebSocket 架构设计文档

## 概述

本项目采用现代化的WebSocket架构设计，使用事件驱动模式和多种设计模式的组合，提供了高性能、可扩展的实时通信解决方案。

## 架构设计原则

1. **单一职责原则**：每个组件只负责一个特定的功能
2. **开闭原则**：对扩展开放，对修改关闭
3. **依赖倒置原则**：依赖抽象而非具体实现
4. **事件驱动**：基于事件的消息处理机制

## 核心架构

### 1. 事件处理器模式 (Event Handler Pattern)

负责处理不同类型的WebSocket消息事件：

```rust
// 消息事件处理器trait
pub trait MessageEventHandler: Send + Sync {
    async fn handle(
        &self,
        message: WebSocketMessage,
        context: &MessageContext,
    ) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>>;
    
    fn supported_message_type(&self) -> &'static str;
}

// 具体实现
pub struct ChatMessageHandler { ... }    // 处理聊天消息
pub struct JoinRoomHandler { ... }       // 处理加入房间
pub struct LeaveRoomHandler { ... }      // 处理离开房间
pub struct ErrorHandler { ... }          // 处理错误消息
```

### 2. 命令模式 (Command Pattern)

将消息处理封装为命令对象：

```rust
pub struct CommandProcessor {
    event_handler_factory: Arc<EventHandlerFactory>,
    broadcast_handler: Arc<Mutex<BroadcastHandler>>,
}

impl CommandProcessor {
    pub async fn process_message(
        &self,
        message: WebSocketMessage,
        ws_sender: &mut SplitSink<...>,
        connection_state: &mut ConnectionState,
    ) -> Result<(), Error> {
        // 根据消息类型获取对应的事件处理器
        // 执行处理逻辑
        // 处理结果
    }
}
```

### 3. 工厂模式 (Factory Pattern)

负责创建和管理事件处理器：

```rust
pub struct EventHandlerFactory {
    handlers: HashMap<String, MessageEventHandlerEnum>,
}

impl EventHandlerFactory {
    pub fn new(
        user_repo: Arc<UserRepository>,
        message_repo: Arc<MessageRepository>,
        session_manager: Arc<SessionManager>,
    ) -> Self {
        // 注册各种事件处理器
    }
    
    pub fn get_handler(&self, message_type: &str) -> Option<&MessageEventHandlerEnum> {
        // 根据消息类型获取处理器
    }
}
```

### 4. 广播模式 (Broadcast Pattern)

处理房间内的消息广播：

```rust
pub struct BroadcastHandler {
    room_channels: HashMap<String, broadcast::Sender<WebSocketMessage>>,
}

impl BroadcastHandler {
    pub fn get_or_create_room_channel(&mut self, room_id: &str) -> broadcast::Sender<WebSocketMessage> {
        // 获取或创建房间广播通道
    }
    
    pub fn should_send_to_client(&self, msg: &WebSocketMessage, current_user_id: &Option<String>) -> bool {
        // 判断是否应该发送给特定客户端
    }
}
```

## 文件结构

```
src/websocket/
├── mod.rs                          # 模块导出
├── message.rs                      # WebSocket消息定义
├── connection_state.rs             # 连接状态管理
├── broadcast_handler.rs            # 广播处理器
├── handler_v2.rs                   # 旧版处理器（已废弃）
├── new_websocket/                  # 新版实现（当前使用）
│   ├── mod.rs
│   ├── handler.rs                  # 主处理器
│   ├── command_processor.rs        # 命令处理器
│   ├── event_handler_factory.rs    # 事件处理器工厂
│   └── event_handlers/             # 事件处理器实现
│       ├── mod.rs
│       ├── message_handler.rs      # 事件处理器trait定义
│       ├── enum_handler.rs         # 事件处理器枚举
│       ├── chat_message_handler.rs # 聊天消息处理器
│       ├── join_room_handler.rs    # 加入房间处理器
│       ├── leave_room_handler.rs   # 离开房间处理器
│       └── error_handler.rs        # 错误处理器
└── old_websocket/                  # 旧版实现（已废弃）
    ├── mod.rs
    ├── handler.rs
    └── message_handlers.rs
```

## 消息处理流程

1. **接收消息**：WebSocket连接接收到消息
2. **解析消息**：将JSON字符串解析为`WebSocketMessage`枚举
3. **命令处理**：`CommandProcessor`根据消息类型获取对应的事件处理器
4. **事件处理**：具体的事件处理器执行业务逻辑
5. **结果处理**：根据处理结果更新连接状态或发送响应
6. **广播消息**：如果需要，通过`BroadcastHandler`广播给房间内其他用户

## 支持的消息类型

| 消息类型 | 处理器 | 功能描述 |
|---------|--------|----------|
| `chat_message` | `ChatMessageHandler` | 处理聊天消息，保存到数据库并广播 |
| `join_room` | `JoinRoomHandler` | 处理用户加入房间，更新在线状态 |
| `leave_room` | `LeaveRoomHandler` | 处理用户离开房间，清理状态 |
| `error` | `ErrorHandler` | 处理错误消息 |

## 扩展新功能

### 添加新的消息类型

1. **定义消息类型**：在`message.rs`中添加新的`WebSocketMessage`变体
2. **创建事件处理器**：在`event_handlers/`目录下创建新的处理器文件
3. **实现trait**：实现`MessageEventHandler` trait
4. **注册处理器**：在`event_handler_factory.rs`中注册新处理器
5. **更新枚举**：在`enum_handler.rs`中添加新的处理器变体

### 示例：添加系统通知消息

```rust
// 1. 在 message.rs 中添加
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    // ... 现有消息类型
    #[serde(rename = "system_notification")]
    SystemNotification { 
        content: String, 
        notification_type: String 
    },
}

// 2. 创建 system_notification_handler.rs
pub struct SystemNotificationHandler;

#[async_trait::async_trait]
impl MessageEventHandler for SystemNotificationHandler {
    async fn handle(&self, message: WebSocketMessage, context: &MessageContext) -> Result<MessageResult, Error> {
        // 处理系统通知逻辑
    }
    
    fn supported_message_type(&self) -> &'static str {
        "system_notification"
    }
}

// 3. 在 event_handler_factory.rs 中注册
handlers.insert(
    "system_notification".to_string(),
    MessageEventHandlerEnum::SystemNotification(SystemNotificationHandler::new()),
);
```

## 性能优化

1. **连接池管理**：使用Arc和Mutex进行线程安全的资源共享
2. **异步处理**：所有I/O操作都是异步的
3. **内存管理**：使用引用计数避免不必要的克隆
4. **广播优化**：使用tokio的broadcast通道进行高效的消息广播

## 错误处理

- **分层错误处理**：每个层级都有相应的错误处理机制
- **错误传播**：使用`Result`类型进行错误传播
- **日志记录**：关键操作都有详细的日志记录
- **优雅降级**：错误发生时系统能够优雅降级

## 测试策略

1. **单元测试**：为每个事件处理器编写单元测试
2. **集成测试**：测试完整的消息处理流程
3. **性能测试**：测试高并发场景下的性能表现
4. **错误测试**：测试各种错误场景的处理

## 配置选项

- **JWT密钥**：通过环境变量`JWT_SECRET`配置
- **数据库连接**：通过环境变量`DATABASE_URL`配置
- **Redis连接**：通过环境变量`REDIS_URL`配置
- **WebSocket端口**：通过环境变量`WS_PORT`配置（默认8301）

## 监控和调试

1. **日志级别**：支持不同级别的日志输出
2. **性能指标**：记录消息处理时间和吞吐量
3. **连接状态**：监控WebSocket连接状态
4. **错误统计**：统计各种错误的发生频率

## 未来扩展

1. **消息持久化**：支持消息的持久化存储
2. **消息队列**：集成消息队列进行异步处理
3. **负载均衡**：支持多实例的负载均衡
4. **消息加密**：支持端到端的消息加密
5. **实时统计**：提供实时的连接和消息统计

---

*本文档最后更新：2025年09月14日
