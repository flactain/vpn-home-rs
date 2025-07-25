use crate::{config::AppState, handlers};
use axum::{Router, routing::get};

pub fn routers() -> Router<AppState> {
    Router::new().route("/", get(handlers::servers_handler::search_all_servers))
}
