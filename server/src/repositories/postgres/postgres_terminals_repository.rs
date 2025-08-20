use std::ops::DerefMut;

use async_trait::async_trait;
use sqlx::{PgPool, Transaction, any::AnyQueryResult};
use vpn_libs::entities::{ids::EntityId, terminals::TerminalOutline};

use crate::repositories::terminals_repository::TerminalsRepository;

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
    async fn exists_by_id(&self, terminal_id: &EntityId) -> sqlx::Result<bool> {
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
                if result_row.is_some() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(err) => Err(err),
        }
    }

    async fn find_by_user_id(&self, owner_user_id: &str) -> sqlx::Result<Vec<TerminalOutline>> {
        sqlx::query_as!(
            TerminalOutline,
            r#"
            SELECT /* terminals.find_by_user_id */
                   terminal_id
                   , terminal_name
                   , owner_user_id
                   , os
                   , created_at
                   , updated_at
              FROM terminals t 
             WHERE 1=1
               AND t.owner_user_id = $1
               AND NOT t.is_deleted
            ;
            "#,
            owner_user_id
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        terminal_info: &TerminalOutline,
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
        .bind(terminal_info.terminal_id.clone())
        .bind(terminal_info.terminal_name.clone())
        .bind(terminal_info.owner_user_id.clone())
        .bind(terminal_info.os.clone())
        .execute(tx.deref_mut())
        .await;

        match result {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(err),
        }
    }
}
