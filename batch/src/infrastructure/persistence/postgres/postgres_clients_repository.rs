use async_trait::async_trait;
use sqlx::PgPool;
use vpn_libs::entities::clients::ClientOutline;

use crate::infrastructure::persistence::clients_repository::ClientsRepository;

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
    async fn find_one(&self, client_outline: ClientOutline) -> sqlx::Result<ClientOutline> {
        let vpn_id: uuid::Uuid = client_outline.vpn_id.into();
        let terminal_id: uuid::Uuid = client_outline.terminal_id.into();

        sqlx::query_as!(
            ClientOutline,
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
}
