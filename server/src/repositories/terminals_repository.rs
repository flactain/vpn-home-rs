use async_trait::async_trait;
use sqlx::{Transaction, any::AnyQueryResult};
use vpn_libs::entities::{ids::EntityId, terminals::Terminal};

#[async_trait]
pub trait TerminalsRepository: Send + Sync {
    async fn exists_by_id(&self, terminal_id: &EntityId) -> sqlx::Result<bool>;
    async fn find_by_user_id(&self, owner_user_id: &str) -> sqlx::Result<Vec<Terminal>>;
    async fn create(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        terminal_info: &Terminal,
    ) -> sqlx::Result<AnyQueryResult, sqlx::Error>;
}
