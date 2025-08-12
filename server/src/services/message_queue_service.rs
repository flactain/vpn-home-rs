
use async_trait::async_trait;
use vpn_libs::entities::messages::MessageType;

use crate::entities::{errors::AppError, ids::EntityId};

#[async_trait]
pub trait MessageService: Send + Sync {
    async fn send(&self, message_type: MessageType, alt_id: EntityId) -> Result<(), AppError>;
}
