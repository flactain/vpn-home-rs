use axum::{Router, routing::get};

use crate::{
    config::AppState,
    handlers::{self, vpns_handler},
};

pub fn routers() -> Router<AppState> {
    let api_routes = Router::new()
        //clients endpoint
        .route(
            "/clients",
            get(handlers::vpns_handler::search_clients)
                .post(handlers::vpns_handler::create_clients),
        )
        //vpns endpoint
        .route("/vpns", get(vpns_handler::search_all_vpns))
        .route(
            "/vpns/requests",
            get(vpns_handler::search_requests).post(vpns_handler::approve_request),
        )
        //servers endpoint
        .route("/servers", get(handlers::vpns_handler::search_all_servers))
        //terminals endpoint
        .route("/terminals", get(handlers::vpns_handler::search_terminals))
        //fallback endpoint
        .fallback(handlers::fallback::fallback_handler);

    Router::new().nest("/api/v1/vpn", api_routes)
}
