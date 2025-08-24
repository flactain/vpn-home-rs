use async_trait::async_trait;
use sqlx::PgPool;
use vpn_libs::entities::ids::EntityId;

use crate::{
    entities::wireguard::HostConfig,
    infrastructure::persistence::servers_repository::ServersRepository,
};

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
    async fn find_server_config(&self, vpn_id: &EntityId) -> sqlx::Result<HostConfig> {
        let vpn_id: uuid::Uuid = vpn_id.into();

        sqlx::query_as!(
            HostConfig,
            r#"
            SELECT /* batch.PostgresServersRepository.find_server_config */
                   s.config_name 
                 , NULL          AS private_key
                 , s.private_ip  AS address
                 , 51280         AS "port!"
              FROM servers s
        INNER JOIN vpns v
                ON s.vpn_id = s.vpn_id 
             WHERE 1=1
               AND s.vpn_id = $1
               AND NOT s.is_deleted 
               AND NOT v.is_deleted 
        ;
        "#,
            vpn_id,
        )
        .fetch_one(&self.pg_pool)
        .await
    }
}
