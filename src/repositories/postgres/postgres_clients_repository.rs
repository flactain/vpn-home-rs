use async_trait::async_trait;
use sqlx::{PgPool, any::AnyQueryResult};

use crate::{
    entities::clients::ClientOutline, repositories::clients_repository::ClientsRepository,
};

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
    async fn find_by_vpn_id(&self, vpn_id: uuid::Uuid) -> sqlx::Result<Vec<ClientOutline>> {
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
            vpn_id
        )
        .fetch_all(&self.pg_pool)
        .await
    }
    async fn create(&self, client_outline: ClientOutline) -> sqlx::Result<AnyQueryResult> {
        Err(sqlx::Error::RowNotFound)
    }
}
