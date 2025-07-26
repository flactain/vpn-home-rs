use async_trait::async_trait;
use sqlx::PgPool;

use crate::{entities::vpns::VpnOutline, repositories::vpns_repository::VpnsRepository};

pub struct PostgresVpnsRepository {
    pub pg_pool: PgPool,
}

impl PostgresVpnsRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        PostgresVpnsRepository { pg_pool }
    }
}

#[async_trait]
impl VpnsRepository for PostgresVpnsRepository {
    async fn find_all(&self) -> Result<Vec<VpnOutline>, sqlx::Error> {
        sqlx::query_as!(
            VpnOutline,
            r#"
                  WITH /* vpns.findAll */ clients_count AS (
                       SELECT 
                              c.vpn_id
                            , count(1) AS clients_count
                       FROM clients c 
                       WHERE 1 = 1
                         AND NOT c.is_deleted
                       GROUP BY c.vpn_id 
                 )
                SELECT 
                       v.vpn_id 
                     , v.vpn_name
                     , v.owner_user_id 
                     , t.terminal_name AS server_name
                     , s.public_ip 
                     , c.clients_count 
                     , v.created_at
                  FROM vpns v
            INNER JOIN servers s 
                    ON v.vpn_id =s.vpn_id 
            INNER JOIN terminals t 
                    ON s.terminal_id = t.terminal_id 
            INNER JOIN clients_count c
                    ON v.vpn_id = c.vpn_id 
                 WHERE 1 = 1
                   AND NOT v.is_deleted
                   AND NOT t.is_deleted 
            ;
        "#
        )
        .fetch_all(&self.pg_pool)
        .await
    }
}
