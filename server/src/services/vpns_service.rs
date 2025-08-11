use std::sync::Arc;

use log::debug;

use crate::{
    entities::{approvals::ApprovalRequest, errors::AppError, vpns::VpnOutlineDto},
    repositories::vpns_repository::VpnsRepository,
};

pub struct VpnsService {
    vpns_repository: Arc<dyn VpnsRepository>,
}

impl VpnsService {
    pub fn new(vpns_repository: Arc<dyn VpnsRepository>) -> Self {
        Self { vpns_repository }
    }

    pub async fn search_all_vpns(&self) -> Result<Vec<VpnOutlineDto>, AppError> {
        debug!("services: search_all_vpns");
        match self.vpns_repository.find_all().await {
            Ok(vpn_outlines) => Ok(vpn_outlines.iter().map(VpnOutlineDto::from).collect()),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn search_requests(&self, user_id: String) -> Result<Vec<ApprovalRequest>, AppError> {
        match self.vpns_repository.find_requests(user_id).await {
            Ok(requests) => Ok(requests),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn approve_vpn(&self, approval_request: ApprovalRequest) -> Result<(), AppError> {
        match self.vpns_repository.approve_vpn(approval_request).await {
            Ok(result) => {
                if result.rows_affected() != 0 {
                    Ok(())
                } else {
                    Err(AppError::AnyhowError(anyhow::anyhow!("something go wrong")))
                }
            }
            Err(err) => Err(AppError::DatabaseError(err)),
        }
    }

    pub async fn approve_client(&self, approval_request: ApprovalRequest) -> Result<(), AppError> {
        match self.vpns_repository.approve_client(approval_request).await {
            Ok(result) => {
                if result.rows_affected() != 0 {
                    Ok(())
                } else {
                    Err(AppError::AnyhowError(anyhow::anyhow!("something go wrong")))
                }
            }
            Err(err) => Err(AppError::DatabaseError(err)),
        }
    }
}
