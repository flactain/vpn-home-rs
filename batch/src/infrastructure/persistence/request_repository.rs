use async_trait::async_trait;
use sqlx::{Transaction, any::AnyQueryResult};
use vpn_libs::entities::clients::ClientOutline;

#[async_trait]
pub trait RequestRepository: Send + Sync {
    async fn approve_client_request(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        client_outline: &ClientOutline,
    ) -> sqlx::Result<AnyQueryResult>;
    async fn find_one_client(&self, client_outline: &ClientOutline) -> sqlx::Result<ClientOutline>;
}
