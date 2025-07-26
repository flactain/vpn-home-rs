use axum::Router;

use crate::{
    config::AppState,
    routes::{servers_router, vpns_router},
};

pub fn routers() -> Router<AppState> {
    Router::new()
        .nest("/api/v1/vpn/servers", servers_router::routers())
        .nest("/api/v1/vpn/vpns", vpns_router::routers())
}
