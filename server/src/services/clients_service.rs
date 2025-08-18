use std::sync::Arc;

use log::debug;
use sqlx::Transaction;
use vpn_libs::entities::messages::MessageType;

use crate::{
    entities::{clients::ClientOutline, errors::AppError, ids::EntityId},
    repositories::clients_repository::ClientsRepository,
    services::message_queue_service::MessageService,
};

pub struct ClientsService {
    clients_repository: Arc<dyn ClientsRepository>,
    message_service: Arc<dyn MessageService>,
}

impl ClientsService {
    pub fn new(
        clients_repository: Arc<dyn ClientsRepository>,
        message_service: Arc<dyn MessageService>,
    ) -> Self {
        ClientsService {
            clients_repository,
            message_service,
        }
    }

    pub async fn search_clients(&self, vpn_id: EntityId) -> Result<Vec<ClientOutline>, AppError> {
        debug!("services: search_clients");

        let result = self.clients_repository.find_by_vpn_id(vpn_id).await;

        match result {
            Ok(client_outlines) => Ok(client_outlines),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(sqlx_err) => Err(sqlx_err.into()),
        }
    }

    pub async fn register_client(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        client_info: ClientOutline,
    ) -> Result<(), AppError> {
        debug!("services: register_client");

        // create client on database
        match self
            .clients_repository
            .create(tx, client_info.clone())
            .await
        {
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

        self.message_service
            .send(MessageType::RequestClient, client_info.terminal_id)
            .await
    }
}
