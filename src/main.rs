use std::sync::Arc;

use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use log::{info, warn};
use vpn_server_rs::{config::{self, AppState}, handlers};

#[tokio::main]
async fn main() {
    // init logger
    env_logger::init();
    info!("start");

    // init config
    let config = config::Config::from_env();
    let state = AppState {
        config: Arc::new(config.unwrap()),
    };

    //build session store
    let session_config = SessionConfig::default().with_table_name("sessions");
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .unwrap();

    // routing
    let app = vpn_server_rs::routes::routers()
        .with_state(state)
        .layer(SessionLayer::new(session_store))
        .fallback(handlers::fallback::fallback_handler);

    // server up
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

