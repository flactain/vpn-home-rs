use std::sync::Arc;

use log::debug;
use sqlx::Transaction;
use vpn_libs::entities::{
    clients::ClientOutline,
    errors::AppError,
    ids::EntityId,
    messages::{MessageType, ResourceHandle, ResourceType},
};

use crate::{
    entities::approvals::ApprovalRequest, repositories::clients_repository::ClientsRepository,
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

        Ok(())

        // sqs enqueue
        // debug!("sqs enqueue!");

        // self.message_service
        //     .send(
        //         MessageType::RequestClient,
        //         serde_json::to_string(&client_info).unwrap(),
        //     )
        //     .await
    }
    pub async fn approve_client(&self, approval_request: ApprovalRequest) -> Result<(), AppError> {
        let client = match self
            .clients_repository
            .find_one(&approval_request.vpn_id, &approval_request.resource_id)
            .await
        {
            Ok(client) => Ok(client),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(AppError::DatabaseError(err)),
        }?;

        let message_type = MessageType::new(ResourceType::Client, ResourceHandle::Create);

        self.message_service
            .send(message_type, serde_json::to_string(&client).unwrap())
            .await
    }
}
