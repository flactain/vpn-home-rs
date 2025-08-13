use async_trait::async_trait;
use sqlx::{Transaction, any::AnyQueryResult};

use crate::entities::{clients::ClientOutline, ids::EntityId};

#[async_trait]
pub trait ClientsRepository: Send + Sync {
    async fn find_by_vpn_id(&self, vpn_id: EntityId) -> sqlx::Result<Vec<ClientOutline>>;
    async fn create(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        client_info: ClientOutline,
    ) -> sqlx::Result<AnyQueryResult>;
}
