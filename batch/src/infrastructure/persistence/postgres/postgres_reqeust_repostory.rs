use std::ops::DerefMut;

use async_trait::async_trait;
use sqlx::{PgPool, Transaction, any::AnyQueryResult};
use vpn_libs::entities::clients::Client;

use crate::infrastructure::persistence::request_repository::RequestRepository;

pub struct PostgresRequestRepository {
    pub pg_pool: PgPool,
}

impl PostgresRequestRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        PostgresRequestRepository { pg_pool }
    }
}

#[async_trait]
impl RequestRepository for PostgresRequestRepository {
    async fn find_one_client(&self, client_outline: &Client) -> sqlx::Result<Client> {
        let vpn_id: uuid::Uuid = client_outline.clone().vpn_id.into();
        let terminal_id: uuid::Uuid = client_outline.clone().terminal_id.into();

        sqlx::query_as!(
            Client,
            r#"
         SELECT /* batch.PostgresRequestRepository.find_one_client()*/
                   c.vpn_id
                 , v.vpn_name 
                 , t.terminal_id 
                 , t.terminal_name 
                 , t.owner_user_id 
                 , c.allowed_ip 
                 , c.public_key 
                 , c.created_at 
                 , c.approved_at IS NOT NULL AS is_approved
              FROM clients c
        INNER JOIN vpns v
                ON v.vpn_id = c.vpn_id 
        INNER JOIN terminals t 
                ON c.terminal_id = t.terminal_id 
             WHERE 1=1
               AND c.vpn_id = $1
               AND c.terminal_id = $2
               AND NOT c.is_deleted 
               AND NOT t.is_deleted
        ;
        "#,
            vpn_id,
            terminal_id
        )
        .fetch_one(&self.pg_pool)
        .await
    }
    async fn approve_client_request(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        client_outline: &Client,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query(
            r#"
        UPDATE /* batch.PostgresRequestRepository.approve_client_request */
               clients c
           SET approved_at   = clock_timestamp()
             , updated_at   = clock_timestamp()
         WHERE 1=1
           AND c.vpn_id      =  $1
           AND c.terminal_id =  $2
           AND c.approved_at IS NULL
           AND NOT c.is_deleted
        ;
        "#,
        )
        .bind(client_outline.vpn_id.clone())
        .bind(client_outline.terminal_id.clone())
        .execute(tx.deref_mut())
        .await;

        match result {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(err),
        }
    }
}
