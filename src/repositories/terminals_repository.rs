use async_trait::async_trait;
use sqlx::any::AnyQueryResult;

use crate::entities::terminals::TerminalOutline;

#[async_trait]
pub trait TerminalsRepository: Send + Sync {
    async fn exists_by_id(&self, terminal_id: uuid::Uuid) -> sqlx::Result<bool>;
    async fn find_by_user_id(&self, owner_user_id: &str) -> sqlx::Result<Vec<TerminalOutline>>;
    async fn create(
        &self,
        terminal_info: TerminalOutline,
    ) -> sqlx::Result<AnyQueryResult, sqlx::Error>;
}
