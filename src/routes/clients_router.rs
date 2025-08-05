use crate::{config::AppState, handlers};
use axum::{Router, routing::get};

pub fn routers() -> Router<AppState> {
    Router::new().route("/", get(handlers::clients_handler::search_clients))
}
