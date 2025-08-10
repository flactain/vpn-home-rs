use std::sync::Arc;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use log::debug;
use uuid::Uuid;
use vpn_libs::entities::messages::{MessageType, SqsMessageBuilder};

use crate::{
    entities::{
        clients::{ClientOutline, ClientOutlineDto},
        errors::AppError,
    },
    repositories::clients_repository::ClientsRepository,
};

pub struct ClientsService {
    clients_repository: Arc<dyn ClientsRepository>,
    sqs_client: Arc<aws_sdk_sqs::Client>,
}

impl ClientsService {
    pub fn new(
        clients_repository: Arc<dyn ClientsRepository>, sqs_client: Arc<aws_sdk_sqs::Client>,
    ) -> Self {
        ClientsService {
            clients_repository,
            sqs_client,
        }
    }

    pub async fn search_clients(&self, vpn_id: &str) -> Result<Vec<ClientOutlineDto>, AppError> {
        debug!("services: search_clients");

        // Validation
        let vpn_id = Uuid::try_from(
            BASE64_URL_SAFE_NO_PAD
                .decode(vpn_id)
                .map_err(|_| AppError::InvalidInput(vpn_id.to_string()))?,
        )
        .map_err(|_| AppError::InvalidInput(vpn_id.to_string()))?;

        let result = self.clients_repository.find_by_vpn_id(vpn_id).await;

        match result {
            Ok(client_outlines) => Ok(client_outlines.iter().map(ClientOutlineDto::from).collect()),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(sqlx_err) => Err(sqlx_err.into()),
        }
    }

    pub async fn register_client(
        &self,
        client_info: ClientOutline,
        queue_url: String,
    ) -> Result<(), AppError> {
        debug!("services: register_client");

        // create client on database
        match self.clients_repository.create(client_info.clone()).await {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(anyhow::anyhow!("failed to join this vpn").into());
                }
            }
            Err(err) => {
                return Err(AppError::DatabaseError(err));
            }
        }

        // sqs enqueue
        debug!("sqs enqueue!");
        let message = SqsMessageBuilder::new()
            .set_message_type(MessageType::CreateClient)
            .set_alt_id(client_info.terminal_id.to_string())
            .build();

        self.sqs_client
            .send_message()
            .queue_url(queue_url)
            .message_body(message.to_string())
            .send()
            .await
            .map_err(|_| anyhow::anyhow!("Failed to Queue a request."))?;

        Ok(())
    }
}
