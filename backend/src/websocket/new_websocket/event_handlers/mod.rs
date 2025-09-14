//! WebSocket 消息事件处理器模块
//!
//! 这个模块包含了处理不同类型WebSocket消息的事件处理器。
//! 每个处理器负责处理特定类型的消息事件。

// pub mod chat_message_handler; // 已删除，使用新的架构
pub mod enum_handler;
pub mod error_handler;
pub mod get_messages_handler;
pub mod get_online_users_handler;
pub mod join_room_handler;
pub mod leave_room_handler;
pub mod message_handler;
pub mod send_message_handler;

// 重新导出主要的类型和trait
// pub use chat_message_handler::ChatMessageHandler; // 已删除
pub use enum_handler::MessageEventHandlerEnum;
pub use error_handler::ErrorHandler;
pub use get_messages_handler::GetMessagesHandler;
pub use get_online_users_handler::GetOnlineUsersHandler;
pub use join_room_handler::JoinRoomHandler;
pub use leave_room_handler::LeaveRoomHandler;
pub use message_handler::{MessageContext, MessageEventHandler, MessageResult};
pub use send_message_handler::SendMessageHandler;
