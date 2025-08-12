use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use vpn_libs::entities::messages::{MessageBuilder, MessageType};

use crate::{
    entities::{errors::AppError, ids::EntityId},
    services::message_queue_service::MessageService,
};

pub struct SqsMessageService {
    sqs_client: Arc<aws_sdk_sqs::Client>,
    general_queue_url: String,
}

impl SqsMessageService {
    pub fn new(sqs_client: Arc<aws_sdk_sqs::Client>, general_queue_url: String) -> Self {
        SqsMessageService {
            sqs_client,
            general_queue_url,
        }
    }

    pub fn get_queue_url(&self, message_type: MessageType) -> String {
        match message_type {
            MessageType::CreateVpn => self.general_queue_url.clone(),
            MessageType::CreateClient => self.general_queue_url.clone(),
            MessageType::ApproveVpn => self.general_queue_url.clone(),
            MessageType::ApproveClient => self.general_queue_url.clone(),
            MessageType::Default => self.general_queue_url.clone(),
        }
    }
}

#[async_trait]
impl MessageService for SqsMessageService {
    async fn send(&self, message_type: MessageType, alt_id: EntityId) -> Result<(), AppError> {
        debug!("sqs enqueue!");
        let message = MessageBuilder::new()
            .set_message_type(message_type)
            .set_alt_id(alt_id.to_string())
            .build();

        self.sqs_client
            .send_message()
            .queue_url(self.get_queue_url(message_type))
            .message_body(message.to_string())
            .send()
            .await
            .map_err(|_| anyhow::anyhow!("Failed to Queue a request."))?;
        Ok(())
    }
}
