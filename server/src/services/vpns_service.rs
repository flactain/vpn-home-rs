use std::sync::Arc;

use log::debug;
use vpn_libs::entities::{
    errors::AppError,
    messages::{MessageType, ResourceHandle, ResourceType},
    vpns::VpnOutline,
};

use crate::{
    entities::approvals::ApprovalRequest, repositories::vpns_repository::VpnsRepository,
    services::message_queue_service::MessageService,
};
pub struct VpnsService {
    vpns_repository: Arc<dyn VpnsRepository>,
    message_service: Arc<dyn MessageService>,
}

impl VpnsService {
    pub fn new(
        vpns_repository: Arc<dyn VpnsRepository>,
        message_service: Arc<dyn MessageService>,
    ) -> Self {
        Self {
            vpns_repository,
            message_service,
        }
    }

    pub async fn search_all_vpns(&self) -> Result<Vec<VpnOutline>, AppError> {
        debug!("services: search_all_vpns");
        match self.vpns_repository.find_all().await {
            Ok(vpn_outlines) => Ok(vpn_outlines),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn search_requests(
        &self,
        user_id: &String,
    ) -> Result<Vec<ApprovalRequest>, AppError> {
        match self.vpns_repository.find_requests(user_id).await {
            Ok(requests) => Ok(requests),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn approve_vpn(&self, approval_request: ApprovalRequest) -> Result<(), AppError> {
        // check existing
        let vpn = match self
            .vpns_repository
            .find_one(&approval_request.vpn_id)
            .await
        {
            Ok(vpn) => Ok(vpn),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(AppError::DatabaseError(err)),
        }?;

        //sqs enqueue
        debug!("vpn sqs approval enqueue");

        let message_type = MessageType::new(ResourceType::Vpn, ResourceHandle::Create);

        self.message_service
            .send(message_type, serde_json::to_string(&vpn).unwrap())
            .await
    }
}
