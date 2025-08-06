use axum::{
    Router,
    routing::get,
};

use crate::{
    config::AppState,
    handlers::{self, vpns_handler},
};

pub fn routers() -> Router<AppState> {
    let api_routes = Router::new()
        //clients endpoint
        .route(
            "/clients",
            get(handlers::clients_handler::search_clients)
                .post(handlers::clients_handler::create_clients),
        )
        //vpns endpoint
        .route("/vpns", get(vpns_handler::search_all_vpns))
        //servers endpoint
        .route(
            "/servers",
            get(handlers::servers_handler::search_all_servers),
        )
        //fallback endpoint
        .fallback(handlers::fallback::fallback_handler);

    Router::new().nest("/api/v1/vpn", api_routes)
}
