use std::sync::Arc;

use async_trait::async_trait;
use aws_sdk_sqs::types::MessageAttributeValue;
use log::debug;
use vpn_libs::entities::{errors::AppError, messages::MessageType};

use crate::services::message_queue_service::MessageService;

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
        self.general_queue_url.clone()
    }
}

#[async_trait]
impl MessageService for SqsMessageService {
    async fn send(&self, message_type: MessageType, message_body: String) -> Result<(), AppError> {
        debug!("sqs enqueue!");
        let resource_type_attr = MessageAttributeValue::builder()
            .data_type("String")
            .string_value(message_type.resource_type.to_string())
            .build()
            .map_err(|_| anyhow::anyhow!("Failed to set MessageType of Request"))?;
        let resource_handle_attr = MessageAttributeValue::builder()
            .data_type("String")
            .string_value(message_type.resource_handle.to_string())
            .build()
            .map_err(|_| anyhow::anyhow!("Failed to set MessageType of Request"))?;

        self.sqs_client
            .send_message()
            .queue_url(self.get_queue_url(message_type))
            .message_body(message_body)
            .message_attributes("resource_type", resource_type_attr)
            .message_attributes("resource_handle", resource_handle_attr)
            .send()
            .await
            .map_err(|_| anyhow::anyhow!("Failed to Queue a request."))?;
        Ok(())
    }
}
