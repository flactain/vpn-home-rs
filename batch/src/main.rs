use std::sync::Arc;

use log::info;
use sqlx::postgres::PgPoolOptions;
use vpn_batch::{
    config::{AppState, Config},
    handlers::message_handler::MessageHandler,
    infrastructure::persistence::postgres::{
        postgres_clients_repository::PostgresClientsRepository,
        postgres_reqeust_repostory::PostgresRequestRepository,
        postgres_servers_repository::PostgresServersRepository,
    },
    listeners::sqs_listener::SqsListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logger
    env_logger::init();
    info!("start");

    //config
    let config = Config::from_env().unwrap();

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(config.database_url.as_str())
        .await
        .unwrap();

    let clients_repository = Arc::new(PostgresClientsRepository::new(pg_pool.clone()));
    let servers_repository = Arc::new(PostgresServersRepository::new(pg_pool.clone()));
    let request_repository = Arc::new(PostgresRequestRepository::new(pg_pool.clone()));

    let message_handler =
        MessageHandler::new(request_repository, clients_repository, servers_repository);

    // AppState
    let state = Arc::new(AppState {
        config: config.clone(),
        pg_pool,
    });
    Config::init_app_state(state).unwrap();

    let listener = SqsListener::new(config.aws_queue_url, message_handler).await;
    listener.listen().await;

    Ok(())
}
