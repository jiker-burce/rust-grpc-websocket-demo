use crate::websocket::WebSocketMessage;
use futures_util::SinkExt;
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message as WsMessage;

/// 广播处理器，负责管理房间广播逻辑
pub struct BroadcastHandler {
    room_channels: HashMap<String, broadcast::Sender<WebSocketMessage>>,
}

impl BroadcastHandler {
    pub fn new() -> Self {
        Self {
            room_channels: HashMap::new(),
        }
    }

    /// 获取或创建房间的广播通道
    pub fn get_or_create_room_channel(
        &mut self,
        room_id: &str,
    ) -> broadcast::Sender<WebSocketMessage> {
        if let Some(sender) = self.room_channels.get(room_id) {
            sender.clone()
        } else {
            let (tx, _) = broadcast::channel(100);
            self.room_channels.insert(room_id.to_string(), tx.clone());
            tx
        }
    }

    /// 获取房间的广播接收器
    pub fn get_room_receiver(
        &self,
        room_id: &str,
    ) -> Option<broadcast::Receiver<WebSocketMessage>> {
        self.room_channels.get(room_id).map(|tx| tx.subscribe())
    }

    /// 获取房间的广播发送器
    pub fn get_room_channel(&self, room_id: &str) -> Option<broadcast::Sender<WebSocketMessage>> {
        self.room_channels.get(room_id).cloned()
    }

    /// 广播消息到指定房间
    pub fn broadcast_to_room(&self, room_id: &str, message: &WebSocketMessage) {
        if let Some(sender) = self.room_channels.get(room_id) {
            let _ = sender.send(message.clone());
        }
    }

    /// 处理广播消息，决定是否发送给客户端
    pub fn should_send_to_client(
        &self,
        message: &WebSocketMessage,
        current_user_id: &Option<String>,
    ) -> bool {
        match message {
            WebSocketMessage::NewMessage { message } => {
                // 新消息：只有不是当前用户发送的才转发
                let msg_user_id = &message.user_id;
                if let Some(current_user) = current_user_id {
                    msg_user_id != current_user
                } else {
                    true
                }
            }
            _ => {
                // 其他类型的消息（如用户上线/下线）直接发送
                true
            }
        }
    }

    /// 发送消息到WebSocket客户端
    pub async fn send_to_client(
        &self,
        ws_sender: &mut tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        message: &WebSocketMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Ok(json) = message.to_json() {
            ws_sender.send(WsMessage::Text(json)).await?;
        }
        Ok(())
    }
}
