use async_trait::async_trait;
use sqlx::any::AnyQueryResult;

use crate::entities::clients::ClientOutline;

#[async_trait]
pub trait ClientsRepository: Send + Sync {
    async fn find_by_vpn_id(&self, vpn_id: uuid::Uuid) -> sqlx::Result<Vec<ClientOutline>>;
    async fn create(&self, client_info: ClientOutline)
    -> sqlx::Result<AnyQueryResult, sqlx::Error>;
}
