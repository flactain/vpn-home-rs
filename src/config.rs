use std::env;

use log::{debug, info};
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct AppState{
    pub config: Config,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub be_app_url: String,
    pub be_app_url_api: String,
    pub fe_app_url: String,
    pub aws_queue_url: String,
    pub aws_region: String,
}
impl Config {
    pub fn from_env() -> Result<Config, envy::Error> {
        let env_name = env::var("RUST_ENV").unwrap_or("local".to_string());
        info!("enviroment: {}", env_name);

        let env_file = match env_name.as_str() {
            "prd" => ".env.prd",
            "dev" => ".env.dev",
            "local" => ".env.local",
            _ => ".env.local",
        };

        dotenvy::dotenv().ok();
        dotenvy::from_path_override(env_file).ok();

        let config = envy::from_env::<Config>();
        debug!("road configs are {:?}", config.as_ref().unwrap());

        config
    }
}
