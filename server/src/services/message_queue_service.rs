use async_trait::async_trait;
use vpn_libs::entities::{errors::AppError, messages::MessageType};

#[async_trait]
pub trait MessageService: Send + Sync {
    async fn send(&self, message_type: MessageType, message_body: String) -> Result<(), AppError>;
}
