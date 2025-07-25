use std::sync::Arc;

use axum::http::{
    HeaderValue, Method,
    header::{self},
};
use axum_cookie::CookieLayer;
use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use log::info;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use vpn_server_rs::{
    config::{AppState, Config},
    handlers,
    repositories::{postgres::postgres_servers_repository::PostgresServersRepository, servers_repository::ServersRepository},
    routes,
    services::servers_service::{self, ServersService},
};

#[tokio::main]
async fn main() {
    // init logger
    env_logger::init();
    info!("start");

    //build session store
    let session_config = SessionConfig::default().with_table_name("sessions");
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .unwrap();

    // config setting
    let config = Config::from_env().unwrap();

    // application setting
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect("")
        .await
        .unwrap();

    // DI container
    let postgres_servers_repository = PostgresServersRepository::new(pool.clone());
    let servers_service = ServersService::new(Arc::new(postgres_servers_repository));

    // app state
    let state = AppState {
        config: Arc::new(config),
        server_service: Arc::new(servers_service),
    };

    // routing
    let app = routes::router::routers()
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
        .fallback(handlers::fallback::fallback_handler);

    // server up
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
