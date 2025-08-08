use aws_config::{BehaviorVersion, Region};
use log::info;
use tokio::task;
use vpn_batch::{config::Config, listeners::sqs_listener::SqsListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logger
    env_logger::init();
    info!("start");

    //config
    let config = Config::from_env().unwrap();

    let listener = SqsListener::new(config.aws_queue_url).await;

    listener.listen().await;

    Ok(())
}
