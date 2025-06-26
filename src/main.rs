use std::sync::Arc;

use axum::{extract::{OriginalUri, State}, http::{StatusCode, Uri}, response::Response};
use log::{info, warn};
use vpn_server_rs::config::{self, AppState};

#[tokio::main]
async fn main() {
    // init logger
    env_logger::init();
    info!("start");

    // init config
    let config = config::Config::from_env();
    let state = AppState{
        config: Arc::new(config.unwrap()),
    };

    // routing
    let app = vpn_server_rs::routes::routers().with_state(state).fallback(fallback_handler);

    // server up
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fallback_handler(OriginalUri(uri):OriginalUri) -> &'static str{
warn!("fallback{}", uri.path());
    "NOT FOUND"
}
