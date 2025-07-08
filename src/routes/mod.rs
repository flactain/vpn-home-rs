use axum::Router;

use crate::config::AppState;

pub fn routers() ->Router<AppState> {
    Router::new()
}
