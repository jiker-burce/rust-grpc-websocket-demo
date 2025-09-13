//! WebSocket 消息事件处理器模块
//!
//! 这个模块包含了处理不同类型WebSocket消息的事件处理器。
//! 每个处理器负责处理特定类型的消息事件。

pub mod chat_message_handler;
pub mod enum_handler;
pub mod error_handler;
pub mod join_room_handler;
pub mod leave_room_handler;
pub mod message_handler;

// 重新导出主要的类型和trait
pub use chat_message_handler::ChatMessageHandler;
pub use enum_handler::MessageEventHandlerEnum;
pub use error_handler::ErrorHandler;
pub use join_room_handler::JoinRoomHandler;
pub use leave_room_handler::LeaveRoomHandler;
pub use message_handler::{MessageContext, MessageEventHandler, MessageResult};
