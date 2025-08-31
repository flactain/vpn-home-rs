use async_trait::async_trait;
use sqlx::{Transaction, any::AnyQueryResult};
use vpn_libs::entities::{clients::Client, ids::EntityId};

#[async_trait]
pub trait ClientsRepository: Send + Sync {
    async fn find_by_vpn_id(&self, vpn_id: EntityId) -> sqlx::Result<Vec<Client>>;
    async fn find_one(&self, vpn_id: &EntityId, terminal_id: &EntityId) -> sqlx::Result<Client>;
    async fn create(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        client_info: Client,
    ) -> sqlx::Result<AnyQueryResult>;
}
