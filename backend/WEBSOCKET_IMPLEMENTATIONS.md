# WebSocket 实现方式说明

本项目提供了两种不同的 WebSocket 实现方式，可以根据需要选择使用：

## 1. 新实现（默认）- 设计模式版本

**位置**: `src/websocket/new_websocket/`

**特点**:
- 使用策略模式（Strategy Pattern）
- 使用命令模式（Command Pattern）  
- 使用工厂模式（Factory Pattern）
- 代码结构清晰，易于扩展
- 每个消息类型都有独立的处理策略
- 支持动态添加新的消息处理策略

**文件结构**:
```
new_websocket/
├── handler.rs              # 主处理器
├── command_processor.rs    # 命令处理器
├── strategy_factory.rs     # 策略工厂
└── strategies/             # 各种策略实现
    ├── mod.rs
    ├── message_strategy.rs
    ├── chat_message_strategy.rs
    ├── join_room_strategy.rs
    ├── leave_room_strategy.rs
    ├── error_strategy.rs
    └── enum_strategy.rs
```

## 2. 旧实现 - 过程式版本

**位置**: `src/websocket/old_websocket/`

**特点**:
- 传统的函数式编程方式
- 所有逻辑集中在一个文件中
- 代码相对简单直接
- 适合小规模项目

**文件结构**:
```
old_websocket/
├── handler.rs           # 主处理器
└── message_handlers.rs  # 消息处理器
```

## 如何切换实现

### 方法1: 使用 Cargo 特性（推荐）

**使用新实现（默认）**:
```bash
cargo run
```

**使用旧实现**:
```bash
cargo run --features old-websocket
```

### 方法2: 修改代码

在 `src/websocket/mod.rs` 中修改导出：

```rust
// 使用新实现
pub use new_websocket::*;

// 或使用旧实现
pub use old_websocket::*;
```

## 性能对比

| 特性 | 新实现 | 旧实现 |
|------|--------|--------|
| 代码可读性 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| 可扩展性 | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| 性能 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 内存使用 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 维护性 | ⭐⭐⭐⭐⭐ | ⭐⭐ |

## 开发建议

- **新项目**: 推荐使用新实现，代码结构更清晰
- **现有项目**: 可以逐步迁移到新实现
- **性能敏感**: 如果对性能要求极高，可以考虑使用旧实现
- **团队开发**: 新实现更适合多人协作开发

## 扩展新功能

### 在新实现中添加新的消息类型

1. 在 `strategies/` 目录下创建新的策略文件
2. 实现 `MessageStrategy` trait
3. 在 `enum_strategy.rs` 中添加新的策略变体
4. 在 `strategy_factory.rs` 中注册新策略
5. 在 `command_processor.rs` 中添加消息类型映射

### 在旧实现中添加新的消息类型

1. 在 `handler.rs` 中添加新的处理函数
2. 在 `handle_websocket_message` 中添加新的匹配分支
3. 在 `message_handlers.rs` 中添加对应的处理逻辑
