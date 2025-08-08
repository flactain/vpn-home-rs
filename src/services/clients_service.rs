use std::sync::Arc;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use log::{debug, error};
use serde_json::json;
use uuid::Uuid;

use crate::{
    entities::{
        clients::{ClientOutline, ClientOutlineDto},
        dto::messages::{MessageType, SqsMessageBuilder},
    },
    repositories::clients_repository::ClientsRepository,
};

pub struct ClientsService {
    clients_repository: Arc<dyn ClientsRepository>,
    sqs_client: Arc<aws_sdk_sqs::Client>,
}

impl ClientsService {
    pub fn new(
        clients_repository: Arc<dyn ClientsRepository>,
        sqs_client: Arc<aws_sdk_sqs::Client>,
    ) -> Self {
        ClientsService {
            clients_repository,
            sqs_client,
        }
    }

    pub async fn search_clients(
        &self,
        vpn_id: &str,
    ) -> Result<Option<Vec<ClientOutlineDto>>, anyhow::Error> {
        debug!("services: search_clients");

        let vpn_id = Uuid::try_from(
            BASE64_URL_SAFE_NO_PAD
                .decode(vpn_id)
                .map_err(|_| anyhow::anyhow!("Invalid vpn_id"))?,
        )
        .map_err(|_| anyhow::anyhow!("Invalid vpn_id"))?;
        let result = self.clients_repository.find_by_vpn_id(vpn_id).await;

        match result {
            Ok(client_outlines) => Ok(Some(
                client_outlines.iter().map(ClientOutlineDto::from).collect(),
            )),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn register_client(
        &self,
        client_info: ClientOutline,
        queue_url: String,
    ) -> Result<(), anyhow::Error> {
        debug!("services: register_client");

        // create client on database
        match self.clients_repository.create(client_info.clone()).await {
            Ok(result) => {
                if result.rows_affected() > 0 {
                } else {
                    return Err(anyhow::anyhow!("failed to join this vpn"));
                }
            }
            Err(err) => {
                error!("{}", err);
                return Err(anyhow::anyhow!("failed to join this vpn"));
            }
        }

        // sqs enqueue
        debug!("sqs enqueue!");
        let message = SqsMessageBuilder::new()
            .set_message_type(MessageType::CreateClient)
            .set_payload(json!({"terminal_id": client_info.terminal_id}))
            .build();

        self.sqs_client
            .send_message()
            .queue_url(queue_url)
            .message_body(message.to_string())
            .send()
            .await?;

        Ok(())
    }
}
