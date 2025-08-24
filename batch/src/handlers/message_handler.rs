use std::sync::Arc;

use vpn_libs::entities::{clients::ClientOutline, errors::AppError};

use crate::{
    config::Config,
    infrastructure::persistence::{
            clients_repository::ClientsRepository, request_repository::RequestRepository,
        },
};

pub struct MessageHandler {
    request_repository: Arc<dyn RequestRepository>,
    client_repository: Arc<dyn ClientsRepository>,
}

impl MessageHandler {
    pub fn new(
        request_repository: Arc<dyn RequestRepository>,
        client_repository: Arc<dyn ClientsRepository>,
    ) -> Self {
        MessageHandler {
            request_repository,
            client_repository,
        }
    }

    pub async fn create_client(&self) -> Result<(), AppError> {
        Ok(())
    }

    pub async fn approve_client(&self, client_outline: String) -> Result<(), AppError> {
        // Convert
        let client_outline: ClientOutline = serde_json::from_str(client_outline.as_str())
            .map_err(|_| AppError::InvalidInput("Failed to Deserialize Client info".to_string()))?;

        let mut tx = Config::get_app_state().pg_pool.begin().await.unwrap();

        // Approve Request on using Repository
        let result = self
            .request_repository
            .approve_client_request(&mut tx, &client_outline)
            .await;
        if result.is_ok() {
            let client_outline = self.client_repository.find_one(client_outline);
        } else if let Err(err) = result {
            return Err(err.into());
        }

        // let host_config = self.servers_repository.find_server_config(vpn_id);
        // let PeerConfig = self.client_repository.find_client_config(vpn_id);

        // wireguard restruct!
        // WireguardClient::new(host_config);

        Ok(())
    }
}
