use async_trait::async_trait;
use vpn_libs::entities::{clients::Client, ids::EntityId};

use crate::entities::wireguard::PeerConfig;

#[async_trait]
pub trait ClientsRepository: Send + Sync {
    async fn find_one(&self, client_outline: Client) -> sqlx::Result<Client>;
    async fn find_client_configs(&self, vpn_id: &EntityId) -> sqlx::Result<Vec<PeerConfig>>;
}
