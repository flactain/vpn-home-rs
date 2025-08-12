use aws_sdk_sqs::{operation::receive_message::ReceiveMessageOutput, types::Message};
use log::{debug, error};
use tokio::task;
use vpn_libs::entities::messages::{AppMessage, MessageType};

use crate::handlers;

pub struct SqsListener {
    queue_url: String,
    sqs_client: aws_sdk_sqs::Client,
}

impl SqsListener {
    pub async fn new(queue_url: String) -> Self {
        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::v2025_01_17())
            .region(aws_config::Region::new(
                std::env::var("AWS_REGION").unwrap(),
            ))
            .load()
            .await;
        let sqs_client = aws_sdk_sqs::Client::new(&aws_config);

        Self {
            queue_url,
            sqs_client,
        }
    }

    pub async fn listen(self) {
        let handle = task::spawn({
            async move {
                self.poll().await;
            }
        });
        handle.await.unwrap();
    }

    async fn poll(&self) {
        loop {
            self.receive().await.ok();
        }
    }

    async fn receive(&self) -> anyhow::Result<()> {
        let receive_messages = self.receive_messages().await?;

        for message in receive_messages.messages() {
            debug!("Got the message: {:?}", message);
            self.process(message).await.ok();
        }

        Ok(())
    }

    async fn process(&self, message: &Message) -> anyhow::Result<()> {
        // message identify
        let receipt_handle = message.receipt_handle().unwrap();
        let message_id = message.message_id().unwrap();
        let message_body: AppMessage =
            serde_json::from_str(message.body().unwrap()).unwrap_or(AppMessage::default());

        // match
        // process
        let process_result = match message_body.message_type() {
            MessageType::CreateClient => handlers::message_handler::create_client().await,
            MessageType::CreateVpn => Ok(()),
            MessageType::ApproveVpn => Ok(()),
            MessageType::ApproveClient => Ok(()),
            MessageType::Default => {
                error!("bad message!");
                Err(anyhow::anyhow!("bad message"))
            }
        };

        // handle message
        match process_result {
            Ok(_) => {
                self.delete_message(receipt_handle).await;
                Ok(())
            }
            Err(err) => {
                self.return_message(receipt_handle).await;
                Err(err)
            }
        }
    }

    async fn receive_messages(&self) -> anyhow::Result<ReceiveMessageOutput> {
        match self
            .sqs_client
            .receive_message()
            .queue_url(self.queue_url.clone())
            .wait_time_seconds(20)
            .max_number_of_messages(1)
            .send()
            .await
        {
            Ok(receive_message) => Ok(receive_message),
            Err(err) => Err(err.into()),
        }
    }

    async fn delete_message(&self, receipt_handle: &str) {
        self.sqs_client
            .delete_message()
            .queue_url(self.queue_url.clone())
            .receipt_handle(receipt_handle)
            .send()
            .await
            .ok();
    }

    async fn return_message(&self, receipt_handle: &str) {
        self.sqs_client
            .change_message_visibility()
            .queue_url(self.queue_url.clone())
            .receipt_handle(receipt_handle)
            .visibility_timeout(0)
            .send()
            .await
            .ok();
    }
}
