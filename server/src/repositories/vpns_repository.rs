use async_trait::async_trait;
use vpn_libs::entities::{ids::EntityId, vpns::VpnOutline};

use crate::entities::approvals::ApprovalRequest;

#[async_trait]
pub trait VpnsRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<VpnOutline>, sqlx::Error>;
    async fn find_one(&self, vpn_id: &EntityId) -> sqlx::Result<VpnOutline>;
    async fn find_requests(&self, user_id: &String) -> Result<Vec<ApprovalRequest>, sqlx::Error>;
}
