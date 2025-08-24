use async_trait::async_trait;
use vpn_libs::entities::ids::EntityId;

use crate::entities::wireguard::HostConfig;

#[async_trait]
pub trait ServersRepository: Send + Sync {
    async fn find_server_config(&self, vpn_id: &EntityId) -> sqlx::Result<HostConfig>;
}
