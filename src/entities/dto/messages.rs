
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SqsMessage {
    message_id: Uuid,
    message_type: MessageType,
    timestamp: chrono::NaiveDateTime,
    alt_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MessageType {
    Default,
    CreateClient,
}

#[derive(Clone)]
pub struct SqsMessageBuilder {
    sqs_message: SqsMessage,
}

impl Default for SqsMessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SqsMessageBuilder {
    pub fn new() -> Self {
        SqsMessageBuilder {
            sqs_message: SqsMessage {
                message_id: Uuid::new_v7(uuid::Timestamp::now(uuid::ContextV7::new())),
                message_type: MessageType::Default,
                timestamp: Local::now().naive_local(),
                alt_id: "".to_string(),
            },
        }
    }

    pub fn set_message_type(&mut self, message_type: MessageType) -> Self {
        self.sqs_message.message_type = message_type;
        self.clone()
    }

    pub fn set_alt_id(&mut self, alt_id: String) -> Self {
        self.sqs_message.alt_id = alt_id;
        self.clone()
    }

    pub fn build(&self) -> serde_json::Value {
        json!(self.clone().sqs_message)
    }
}
