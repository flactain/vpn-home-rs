use async_trait::async_trait;
use sqlx::{PgPool, any::AnyQueryResult};

use crate::{
    entities::terminals::TerminalOutline, repositories::terminals_repository::TerminalsRepository,
};

pub struct PostgresTerminalsRepository {
    pub pg_pool: PgPool,
}

impl PostgresTerminalsRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        PostgresTerminalsRepository { pg_pool }
    }
}

#[async_trait]
impl TerminalsRepository for PostgresTerminalsRepository {
    async fn exists_by_id(&self, terminal_id: uuid::Uuid) -> sqlx::Result<bool> {
        Err(sqlx::Error::RowNotFound)
    }
    async fn find_by_id(&self, terminal_id: uuid::Uuid) -> sqlx::Result<Vec<TerminalOutline>> {
        Err(sqlx::Error::RowNotFound)
    }
    async fn create(
        &self,
        terminal_info: TerminalOutline,
    ) -> sqlx::Result<AnyQueryResult, sqlx::Error> {
        Err(sqlx::Error::RowNotFound)
    }
}
