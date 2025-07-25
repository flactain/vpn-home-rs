use std::sync::Arc;

use axum::Router;
use sqlx::postgres::PgPoolOptions;

use crate::{config::{AppState, Config}, repositories::postgres::postgres_servers_repository::{self, PostgresServersRepository}, routes::servers_router, services::servers_service::ServersService};


pub fn routers() ->Router<AppState> {
    Router::new()
        .nest("/api/v1/vpn/servers", servers_router::routers())
}
