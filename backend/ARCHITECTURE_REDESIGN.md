# WebSocket 架构重新设计

## 当前问题分析

现有的实现将事件处理器误称为"策略"，这不符合设计模式的原则：

### 当前实现的问题
1. **命名不当**：`JoinRoomStrategy` 等实际上是事件处理器，不是策略
2. **职责混乱**：每个"策略"处理特定事件类型，而非提供不同算法
3. **缺乏策略本质**：策略模式应为同一问题提供不同解决方案

## 重新设计的架构

### 1. 事件处理器 (Event Handlers)
负责处理不同类型的WebSocket消息事件：

```rust
// 消息事件处理器
pub trait MessageEventHandler {
    async fn handle(&self, message: WebSocketMessage, context: &MessageContext) -> Result<MessageResult, Error>;
    fn supported_message_type(&self) -> &'static str;
}

// 具体实现
pub struct ChatMessageHandler { ... }
pub struct JoinRoomHandler { ... }
pub struct LeaveRoomHandler { ... }
```

### 2. 策略模式 (Strategy Pattern)
为同一问题提供不同的解决方案：

```rust
// 消息发送策略
pub trait MessageSendingStrategy {
    async fn send_message(&self, message: &WebSocketMessage, context: &MessageContext) -> Result<(), Error>;
}

// 具体策略实现
pub struct ImmediateSendingStrategy { ... }      // 立即发送
pub struct BatchSendingStrategy { ... }          // 批量发送
pub struct DelayedSendingStrategy { ... }        // 延迟发送

// 用户认证策略
pub trait AuthenticationStrategy {
    async fn authenticate(&self, token: &str) -> Result<User, Error>;
}

// 具体认证策略
pub struct JWTStrategy { ... }                   // JWT认证
pub struct SessionStrategy { ... }               // Session认证
pub struct OAuthStrategy { ... }                 // OAuth认证

// 消息存储策略
pub trait MessageStorageStrategy {
    async fn store_message(&self, message: &ChatMessage) -> Result<(), Error>;
}

// 具体存储策略
pub struct DatabaseStorageStrategy { ... }       // 数据库存储
pub struct MemoryStorageStrategy { ... }         // 内存存储
pub struct HybridStorageStrategy { ... }         // 混合存储
```

### 3. 组合架构
主处理器使用事件处理器+策略模式的组合：

```rust
pub struct WebSocketHandler {
    event_handlers: HashMap<String, Box<dyn MessageEventHandler>>,
    sending_strategy: Box<dyn MessageSendingStrategy>,
    auth_strategy: Box<dyn AuthenticationStrategy>,
    storage_strategy: Box<dyn MessageStorageStrategy>,
}
```

## 实现步骤

1. **重命名现有文件**：将 `strategies/` 目录重命名为 `event_handlers/`
2. **创建真正的策略**：实现消息发送、认证、存储等策略
3. **重构主处理器**：使用事件处理器+策略的组合
4. **保持向后兼容**：通过配置选择不同的策略实现

## 优势

1. **职责清晰**：事件处理器处理事件，策略提供算法选择
2. **易于扩展**：可以轻松添加新的策略实现
3. **符合设计模式**：真正实现了策略模式的设计意图
4. **配置灵活**：可以通过配置选择不同的策略组合
