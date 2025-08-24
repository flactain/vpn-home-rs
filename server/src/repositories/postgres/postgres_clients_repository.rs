use std::ops::DerefMut;

use async_trait::async_trait;
use sqlx::{PgPool, Transaction, any::AnyQueryResult};
use vpn_libs::entities::{clients::ClientOutline, ids::EntityId};

use crate::repositories::clients_repository::ClientsRepository;

pub struct PostgresClientsRepository {
    pub pg_pool: PgPool,
}

impl PostgresClientsRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        PostgresClientsRepository { pg_pool }
    }
}

#[async_trait]
impl ClientsRepository for PostgresClientsRepository {
    async fn find_by_vpn_id(&self, vpn_id: EntityId) -> sqlx::Result<Vec<ClientOutline>> {
        sqlx::query_as!(
            ClientOutline,
            r#"
                 SELECT /* clients.findClients */
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
                     ON c.vpn_id = v.vpn_id
             INNER JOIN terminals t
                     ON c.terminal_id = t.terminal_id
                  WHERE 1 = 1
                    AND v.vpn_id = $1
                AND NOT c.is_deleted
                AND NOT t.is_deleted 
             ;
            "#,
            uuid::Uuid::from(vpn_id)
        )
        .fetch_all(&self.pg_pool)
        .await
    }
    async fn find_one(
        &self,
        vpn_id: &EntityId,
        terminal_id: &EntityId,
    ) -> sqlx::Result<ClientOutline> {
        let vpn_id: uuid::Uuid = vpn_id.into();
        let terminal_id: uuid::Uuid = terminal_id.into();

        sqlx::query_as!(
            ClientOutline,
            r#"
                 SELECT /* server.clients.find_one*/
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
                     ON c.vpn_id = v.vpn_id
             INNER JOIN terminals t
                     ON c.terminal_id = t.terminal_id
                  WHERE 1 = 1
                    AND v.vpn_id      = $1
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

    async fn create(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        client_info: ClientOutline,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query(
            r#"
        INSERT INTO clients (
                    vpn_id
                  , terminal_id
                  , allowed_ip
                  , public_key
                  , approved_at
                  , created_at
                  , updated_at
                  , is_deleted )
             VALUES ( 
                    $1
                  , $2
                  , $3
                  , $4
                  , NULL 
                  , CURRENT_TIMESTAMP
                  , CURRENT_TIMESTAMP
                  , FALSE )
        ;
        "#,
        )
        .bind(client_info.vpn_id)
        .bind(client_info.terminal_id)
        .bind(client_info.allowed_ip)
        .bind(client_info.public_key)
        .execute(tx.deref_mut())
        .await;

        match result {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(err),
        }
    }
}
