use aws_sdk_sqs::{operation::receive_message::ReceiveMessageOutput, types::Message};
use log::{debug, error};
use tokio::task;
use vpn_libs::entities::{
    clients::Client,
    errors::AppError,
    messages::{ResourceHandle, ResourceType},
};

use crate::handlers::message_handler::MessageHandler;

pub struct SqsListener {
    queue_url: String,
    dlq_url: String,
    sqs_client: aws_sdk_sqs::Client,
    message_handler: MessageHandler,
}

impl SqsListener {
    pub async fn new(queue_url: String, dlq_url: String, message_handler: MessageHandler) -> Self {
        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::v2025_01_17())
            .region(aws_config::Region::new(
                std::env::var("AWS_REGION").unwrap(),
            ))
            .load()
            .await;
        let sqs_client = aws_sdk_sqs::Client::new(&aws_config);

        Self {
            queue_url,
            dlq_url,
            sqs_client,
            message_handler,
        }
    }

    pub async fn listen(self) {
        let handle = task::spawn({
            async move {
                self.poll().await;
            }
        });
        handle.await.ok();
    }

    async fn poll(&self) {
        loop {
            let receive_messages = self.receive_messages().await.unwrap();

            for message in receive_messages.messages() {
                debug!("Got the message: {:?}", message);
                self.process(message)
                    .await
                    .map_err(|error| error!("{}", error))
                    .ok();
            }
        }
    }

    async fn process(&self, message: &Message) -> Result<(), AppError> {
        // extract message meta data
        let receipt_handle =
            message
                .receipt_handle()
                .ok_or(AppError::AnyhowError(anyhow::anyhow!(
                    "sqs message has no receipt handle"
                )))?;
        let _message_id = message
            .message_id()
            .ok_or(AppError::AnyhowError(anyhow::anyhow!(
                "sqs message has no message id"
            )))?;

        match self.handle(message).await {
            // handle message
            Ok(_) => {
                self.delete_message(receipt_handle).await?;
                Ok(())
            }
            Err(AppError::InvalidInput(err)) => {
                self.send_to_dql(message).await?;
                Err(AppError::InvalidInput(err))
            }
            Err(AppError::AnyhowError(err)) => {
                self.send_to_dql(message).await?;
                Err(AppError::AnyhowError(err))
            }
            Err(err) => {
                self.return_message(receipt_handle).await?;
                Err(err)
            }
        }
    }

    async fn handle(&self, message: &Message) -> Result<(), AppError> {
        let message_attributes = message.message_attributes().ok_or_else(|| {
            AppError::InvalidInput("sqs message has no message attributes".to_string())
        })?;

        let resource_type: ResourceType = message_attributes
            .get("resource_type")
            .ok_or_else(|| {
                AppError::InvalidInput("message_attributes.resource_type.Key".to_string())
            })?
            .string_value()
            .ok_or_else(|| {
                AppError::InvalidInput("message_attributes.resource_type.Value".to_string())
            })?
            .parse()?;
        let resource_handle: ResourceHandle = message_attributes
            .get("resource_handle")
            .ok_or_else(|| {
                AppError::InvalidInput("message_attributes.resource_handle.Key".to_string())
            })?
            .string_value()
            .ok_or_else(|| {
                AppError::InvalidInput("message_attributes.resource_handle.Value".to_string())
            })?
            .parse()?;
        //TODO: switch when resource_type
        let client: Client = serde_json::from_str(
            message
                .body()
                .ok_or_else(|| AppError::InvalidInput("empty(message body)".to_string()))?,
        )
        .map_err(|_| AppError::InvalidInput("message_body".to_string()))?;

        // match
        // process
        match resource_type {
            ResourceType::Client => match resource_handle {
                ResourceHandle::Create => Ok(()),
                ResourceHandle::Edit => Ok(()),
                ResourceHandle::Delete => Ok(()),
                ResourceHandle::Archive => Ok(()),
                ResourceHandle::Approve => self.message_handler.approve_client(client).await,
            },
            ResourceType::Vpn => Ok(()),
        }
    }

    async fn receive_messages(&self) -> anyhow::Result<ReceiveMessageOutput> {
        match self
            .sqs_client
            .receive_message()
            .queue_url(self.queue_url.clone())
            .wait_time_seconds(20)
            .max_number_of_messages(1)
            .message_attribute_names("resource_handle")
            .message_attribute_names("resource_type")
            .send()
            .await
        {
            Ok(receive_message) => Ok(receive_message),
            Err(err) => Err(err.into()),
        }
    }

    async fn delete_message(&self, receipt_handle: &str) -> anyhow::Result<()> {
        self.sqs_client
            .delete_message()
            .queue_url(self.queue_url.clone())
            .receipt_handle(receipt_handle)
            .send()
            .await?;

        Ok(())
    }

    async fn return_message(&self, receipt_handle: &str) -> anyhow::Result<()> {
        self.sqs_client
            .change_message_visibility()
            .queue_url(self.queue_url.clone())
            .receipt_handle(receipt_handle)
            .visibility_timeout(5)
            .send()
            .await?;

        Ok(())
    }

    async fn send_to_dql(&self, message: &Message) -> anyhow::Result<()> {
        self.sqs_client
            .send_message()
            .queue_url(self.dlq_url.clone())
            .set_message_body(message.body().map(|s| s.to_string()))
            .set_message_attributes(message.message_attributes().map(|m| m.to_owned()))
            .send()
            .await?;

        Ok(())
    }
}
