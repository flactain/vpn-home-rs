use std::sync::Arc;

use axum::{
    http::{
        HeaderValue, Method,
        header::{self},
    },
    routing::get,
};
use axum_cookie::CookieLayer;
use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use log::info;
use tower_http::cors::CorsLayer;
use vpn_auth::{
    config::{self, AppState},
    handlers,
};

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
    let app = vpn_auth::routes::routers()
        .with_state(state.clone())
        .layer(CookieLayer::default())
        .layer(SessionLayer::new(session_store))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    state
                        .clone()
                        .config
                        .fe_app_url
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([header::CONTENT_TYPE, header::ACCEPT, header::AUTHORIZATION]),
        )
        .fallback(handlers::fallback::fallback_handler)
        .route("/home", get(|| async { "hello" }));

    // server up
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
