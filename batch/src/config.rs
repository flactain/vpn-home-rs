use std::{
    env,
    sync::{Arc, OnceLock},
};

use log::{debug, info};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub pg_pool: PgPool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub wg_conf_dir: String,
    pub aws_queue_url: String,
    pub aws_dlq_url: String,
    pub aws_region: String,
}
pub static APP_STATE: OnceLock<Arc<AppState>> = OnceLock::new();

impl Config {
    pub fn get_app_state() -> &'static Arc<AppState> {
        APP_STATE.get().unwrap()
    }
    pub fn init_app_state(app_state: Arc<AppState>) -> Result<(), Arc<AppState>> {
        APP_STATE.set(app_state)
    }

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
