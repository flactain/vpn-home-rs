use async_trait::async_trait;
use vpn_libs::entities::servers::ServerEntity;

#[async_trait]
pub trait ServersRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<ServerEntity>, sqlx::Error>;
}
