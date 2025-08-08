use std::sync::{Arc, OnceLock};

use aws_config::{BehaviorVersion, Region};
use log::info;
use tokio::task;
use vpn_batch_common_rs::{config::{AppState, Config}, listeners::sqs_listener::SqsListener};

static APP_STATE: OnceLock<Arc<AppState>> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logger
    env_logger::init();
    info!("start");

    //config
    let config = Config::from_env().unwrap();

    // AppState
    let state = Arc::new(AppState{
        config: config.clone()
    });
    APP_STATE.set(state).unwrap();

    // set listener
    let listener = SqsListener::new(config.aws_queue_url).await;
    listener.listen().await;

    Ok(())
}
