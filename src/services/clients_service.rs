use std::sync::Arc;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use log::debug;
use uuid::Uuid;

use crate::{
    entities::clients::{ClientCreate, ClientOutlineDto},
    repositories::clients_repository::ClientsRepository,
};

pub struct ClientsService {
    clients_repository: Arc<dyn ClientsRepository>,
}

impl ClientsService {
    pub fn new(clients_repository: Arc<dyn ClientsRepository>) -> Self {
        ClientsService { clients_repository }
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

    pub async fn register_client(&self, client_info: ClientCreate) -> Result<(), anyhow::Error> {
        debug!("services: register_client");

        match self.clients_repository.create(client_info).await {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("failed to join this vpn"))
                }
            }
            Err(err) => {
                debug!("{}", err);
                Err(anyhow::anyhow!("failed to join this vpn"))
            }
        }
    }
}
