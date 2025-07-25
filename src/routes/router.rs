
use axum::Router;

use crate::{config::AppState, routes::servers_router};


pub fn routers() ->Router<AppState> {
    Router::new()
        .nest("/api/v1/vpn/servers", servers_router::routers())
}
