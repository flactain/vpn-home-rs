use async_trait::async_trait;
use sqlx::PgPool;

use crate::{entities::servers::ServerEntity, repositories::servers_repository::ServersRepository};

pub struct PostgresServersRepository {
    pub pg_pool: PgPool,
}

impl PostgresServersRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        PostgresServersRepository { pg_pool }
    }
}

#[async_trait]
impl ServersRepository for PostgresServersRepository {
    async fn find_all(&self) -> Result<Vec<ServerEntity>, sqlx::Error> {
        sqlx::query_as!(
            ServerEntity,
            r#"
                SELECT /* servers.findAll */
                  s.vpn_id
                , v.vpn_name 
                , s.terminal_id 
                , t.terminal_name 
                , t.owner_user_id 
                , s.public_ip 
                , s.private_ip
                , s.created_at 
                , v.approved_at IS NOT NULL AS is_approved
                  FROM servers s
            INNER JOIN vpns v
                    ON s.vpn_id = v.vpn_id 
            INNER JOIN terminals t 
                    ON s.terminal_id =t.terminal_id 
            INNER JOIN users u
                    ON t.owner_user_id = u.user_id
            ;
        "#
        )
        .fetch_all(&self.pg_pool)
        .await
    }
}
