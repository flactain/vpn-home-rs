use std::sync::Arc;

use vpn_libs::entities::{clients::Client, errors::AppError};

use crate::{
    config::Config,
    infrastructure::{
        external::wireguard::WireguardClient,
        persistence::{
            clients_repository::ClientsRepository, request_repository::RequestRepository,
            servers_repository::ServersRepository,
        },
    },
};

pub struct MessageHandler {
    request_repository: Arc<dyn RequestRepository>,
    client_repository: Arc<dyn ClientsRepository>,
    servers_repository: Arc<dyn ServersRepository>,
}

impl MessageHandler {
    pub fn new(
        request_repository: Arc<dyn RequestRepository>,
        client_repository: Arc<dyn ClientsRepository>,
        servers_repository: Arc<dyn ServersRepository>,
    ) -> Self {
        MessageHandler {
            request_repository,
            client_repository,
            servers_repository,
        }
    }

    pub async fn create_client(&self) -> Result<(), AppError> {
        Ok(())
    }

    pub async fn approve_client(&self, client_outline: Client) -> Result<(), AppError> {
        let mut tx = Config::get_app_state().pg_pool.begin().await.unwrap();

        // Approve Request on using Repository
        let result = self
            .request_repository
            .approve_client_request(&mut tx, &client_outline)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        // get configurations
        let host_config = self
            .servers_repository
            .find_server_config(&client_outline.vpn_id)
            .await?;
        let peer_configs = self
            .client_repository
            .find_client_configs(&client_outline.vpn_id)
            .await?;

        // wireguard restruct!
        // TODO do i have to reconstruct intentionally?
        let wireguard_client = WireguardClient::new(host_config)?;
        wireguard_client.add_peers(peer_configs)?;

        Ok(())
    }
}
