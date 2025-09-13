// 共享的WebSocket组件
pub mod broadcast_handler;
pub mod connection_state;
pub mod message;

// 两种不同的实现方式
pub mod new_websocket; // 重构后的设计模式实现
pub mod old_websocket; // 原始过程式实现

// 根据环境变量选择实现方式
// 设置 WEBSOCKET_IMPL=new 使用新实现（默认）
// 设置 WEBSOCKET_IMPL=old 使用旧实现
#[cfg(feature = "old-websocket")]
pub use old_websocket::*;

#[cfg(not(feature = "old-websocket"))]
pub use new_websocket::*;

// 共享组件导出
pub use broadcast_handler::*;
pub use connection_state::*;
pub use message::*;
