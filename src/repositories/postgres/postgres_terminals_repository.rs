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
        let result = sqlx::query(
            r#"
        SELECT 1 
          FROM terminals t
         WHERE 1=1
           AND t.terminal_id = $1
        ;
        "#,
        )
        .bind(terminal_id)
        .fetch_optional(&self.pg_pool)
        .await;

        match result {
            Ok(result_row) => {
                if let Some(_) = result_row {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(err) => Err(err),
        }
    }

    async fn find_by_id(&self, terminal_id: uuid::Uuid) -> sqlx::Result<Vec<TerminalOutline>> {
        Err(sqlx::Error::RowNotFound)
    }
    async fn create(
        &self,
        terminal_info: TerminalOutline,
    ) -> sqlx::Result<AnyQueryResult, sqlx::Error> {
        let result = sqlx::query(
            r#"
        INSERT INTO terminals (
                    terminal_id
                  , terminal_name
                  , owner_user_id
                  , os
                  , is_deleted
                  , created_at
                  , updated_at
                  )
             VALUES (
                    $1
                  , $2
                  , $3
                  , $4
                  , FALSE
                  , CURRENT_TIMESTAMP
                  , CURRENT_TIMESTAMP
                  )
        ;
        "#,
        )
        .bind(terminal_info.terminal_id)
        .bind(terminal_info.terminal_name)
        .bind(terminal_info.owner_user_id)
        .bind(terminal_info.os)
        .execute(&self.pg_pool)
        .await;

        match result {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(err),
        }
    }
}
