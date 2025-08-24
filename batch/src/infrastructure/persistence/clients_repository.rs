use async_trait::async_trait;
use vpn_libs::entities::clients::ClientOutline;

#[async_trait]
pub trait ClientsRepository: Send + Sync {
    async fn find_one(&self, client_outline: ClientOutline) -> sqlx::Result<ClientOutline>;
}
