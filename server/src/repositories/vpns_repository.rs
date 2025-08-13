use async_trait::async_trait;
use sqlx::any::AnyQueryResult;

use crate::entities::{approvals::ApprovalRequest, vpns::VpnOutline};

#[async_trait]
pub trait VpnsRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<VpnOutline>, sqlx::Error>;
    async fn find_requests(&self, user_id: &String) -> Result<Vec<ApprovalRequest>, sqlx::Error>;
    async fn approve_vpn(&self, approval_request: ApprovalRequest) -> sqlx::Result<AnyQueryResult>;
    async fn approve_client(
        &self,
        approval_request: ApprovalRequest,
    ) -> sqlx::Result<AnyQueryResult>;
}
