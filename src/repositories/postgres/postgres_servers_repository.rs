use async_trait::async_trait;
use sqlx::PgPool;

use crate::repositories::servers_repository::ServersRepository;

pub struct PostgresServersRepository{
    pub pg_pool: PgPool,
}

impl PostgresServersRepository {
    pub fn new(pg_pool:PgPool)->Self{
        PostgresServersRepository { pg_pool: pg_pool }
    }
}

#[async_trait]
impl ServersRepository for PostgresServersRepository {
    async fn find_all(&self){

    }
}
