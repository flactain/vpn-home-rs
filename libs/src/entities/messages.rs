use core::fmt;

use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppMessage {
    message_id: Uuid,
    message_type: MessageType,
    timestamp: chrono::NaiveDateTime,
    alt_id: String,
}

impl fmt::Display for AppMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "message_id:{}, message_type:{}, timestamp:{}, alt_id:{}",
            self.message_id, self.message_type, self.timestamp, self.alt_id
        )
    }
}

impl AppMessage {
    pub fn message_id(&self) -> Uuid {
        self.message_id
    }
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
    pub fn timestamp(&self) -> chrono::NaiveDateTime {
        self.timestamp
    }
    pub fn alt_id(&self) -> String {
        self.alt_id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MessageType {
    Default,
    RequestVpn,
    RequestClient,
    ApproveClient,
    ApproveVpn,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[repr(u32)]
pub enum MessagePriority {
    VeryHigh = 100,
    High = 80,
    Normal = 60,
    Low = 40,
    VeryLow = 20,
    Unnecesarry = 0,
}

#[derive(Clone)]
pub struct MessageBuilder {
    message: AppMessage,
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {
            message: AppMessage {
                message_id: Uuid::new_v7(uuid::Timestamp::now(uuid::ContextV7::new())),
                message_type: MessageType::Default,
                timestamp: Local::now().naive_local(),
                alt_id: "".to_string(),
            },
        }
    }

    pub fn set_message_type(&mut self, message_type: MessageType) -> Self {
        self.message.message_type = message_type;
        self.clone()
    }

    pub fn set_alt_id(&mut self, alt_id: String) -> Self {
        self.message.alt_id = alt_id;
        self.clone()
    }

    pub fn build(&self) -> serde_json::Value {
        json!(self.clone().message)
    }
}
