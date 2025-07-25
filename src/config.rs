use log::{debug, info};
use std::{env, sync::Arc};
use serde::Deserialize;
use crate::services::servers_service::ServersService;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub server_service: Arc<ServersService>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub be_app_url: String,
    pub be_app_url_api: String,
    pub fe_app_url: String,
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

        let _ = dotenvy::dotenv();
        let _ = dotenvy::from_path_override(env_file);

        let config = envy::from_env::<Config>();
        debug!("road configs are {:?}", config.as_ref().unwrap());

        config
    }
}
