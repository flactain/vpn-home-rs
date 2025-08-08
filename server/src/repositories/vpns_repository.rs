use async_trait::async_trait;

use crate::entities::vpns::VpnOutline;

#[async_trait]
pub trait VpnsRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<VpnOutline>, sqlx::Error>;
}
