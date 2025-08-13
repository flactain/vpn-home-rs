use std::sync::Arc;

use aws_config::{BehaviorVersion, Region};
use axum::http::{
    HeaderValue, Method,
    header::{self},
};
use axum_cookie::CookieLayer;
use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use log::info;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use vpn_server::{
    config::{AppState, Config},
    repositories::postgres::{
        postgres_clients_repository::PostgresClientsRepository,
        postgres_servers_repository::PostgresServersRepository,
        postgres_terminals_repository::PostgresTerminalsRepository,
        postgres_vpns_repository::PostgresVpnsRepository,
    },
    routes,
    services::{
        clients_service::ClientsService, externals::message_sqs_service::SqsMessageService,
        servers_service::ServersService, terminals_service::TerminalsService,
        vpns_service::VpnsService,
    },
};

#[tokio::main]
async fn main() {
    // init logger
    env_logger::init();
    info!("start");

    // config setting
    let config = Config::from_env().unwrap();

    // aws setting
    let aws_sdk_config = aws_config::defaults(BehaviorVersion::v2025_01_17())
        .region(Region::new(config.clone().aws_region))
        .load()
        .await;

    let sqs_client = aws_sdk_sqs::Client::new(&aws_sdk_config);

    //build session store
    let session_config = SessionConfig::default().with_table_name("sessions");
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .unwrap();

    // application setting
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(config.clone().database_url.as_str())
        .await
        .unwrap();

    // DI container(repository)
    let postgres_servers_repository = PostgresServersRepository::new(pool.clone());
    let postgres_vpns_repository = PostgresVpnsRepository::new(pool.clone());
    let postgres_clients_repository = PostgresClientsRepository::new(pool.clone());
    let postgres_terminals_repository = PostgresTerminalsRepository::new(pool.clone());
    // DI container (external)
    let message_sqs_service =
        SqsMessageService::new(Arc::new(sqs_client), config.clone().aws_queue_url);
    //DI container (service)
    let servers_service = ServersService::new(Arc::new(postgres_servers_repository));
    let vpns_service = VpnsService::new(Arc::new(postgres_vpns_repository));
    let clients_service = ClientsService::new(
        Arc::new(postgres_clients_repository),
        Arc::new(message_sqs_service),
    );
    let terminals_service = TerminalsService::new(Arc::new(postgres_terminals_repository));

    // app state
    let state = AppState {
        config: Arc::new(config),
        server_service: Arc::new(servers_service),
        vpns_service: Arc::new(vpns_service),
        clients_service: Arc::new(clients_service),
        terminals_service: Arc::new(terminals_service),
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
        );

    let s = "019841d5-83cb-79bb-83a4-100b6a89561e";
    let s = uuid::Uuid::parse_str(s).unwrap();

    println!("vpn_id: {}", BASE64_URL_SAFE_NO_PAD.encode(s));

    // server up
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
