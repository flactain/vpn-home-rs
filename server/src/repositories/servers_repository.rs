use async_trait::async_trait;

use crate::entities::servers::ServerOutline;

#[async_trait]
pub trait ServersRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<ServerOutline>, sqlx::Error>;
}
