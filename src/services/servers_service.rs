use std::sync::Arc;

use log::debug;

use crate::{
    entities::servers::ServerOutlineDto,
    repositories::servers_repository::ServersRepository,
};

pub struct ServersService {
    servers_repository: Arc<dyn ServersRepository>,
}

impl ServersService {
    pub fn new(servers_repository: Arc<dyn ServersRepository>) -> Self {
        Self { servers_repository }
    }
    pub async fn search_all_servers(&self) -> Result<Option<Vec<ServerOutlineDto>>, anyhow::Error> {
        debug!("services: search_all_servers");
        match self.servers_repository.find_all().await {
            // Ok(server_outlines) => Ok(Some(ServerOutlineDto::from(server_outlines))),
            Ok(server_outlines) => Ok(Some(
                server_outlines
                    .iter()
                    .map(ServerOutlineDto::from)
                    .collect(),
            )),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
