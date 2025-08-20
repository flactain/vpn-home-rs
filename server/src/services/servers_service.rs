use std::sync::Arc;

use log::debug;
use vpn_libs::entities::{errors::AppError, servers::ServerEntity};

use crate::repositories::servers_repository::ServersRepository;

pub struct ServersService {
    servers_repository: Arc<dyn ServersRepository>,
}

impl ServersService {
    pub fn new(servers_repository: Arc<dyn ServersRepository>) -> Self {
        Self { servers_repository }
    }
    pub async fn search_all_servers(&self) -> Result<Vec<ServerEntity>, AppError> {
        debug!("services: search_all_servers");
        match self.servers_repository.find_all().await {
            Ok(server_outlines) => Ok(server_outlines),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(err.into()),
        }
    }
}
