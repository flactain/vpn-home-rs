use axum::{Router, routing::get};

use crate::{config::AppState, handlers::vpns_handler};

pub fn routers() -> Router<AppState> {
    Router::new().route("/", get(vpns_handler::search_all_vpns))
}
