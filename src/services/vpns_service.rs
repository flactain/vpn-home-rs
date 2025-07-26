use std::sync::Arc;

use log::debug;

use crate::{entities::vpns::VpnOutline, repositories::vpns_repository::VpnsRepository};

pub struct VpnsService {
    vpns_repository: Arc<dyn VpnsRepository>,
}

impl VpnsService {
    pub fn new(vpns_repository: Arc<dyn VpnsRepository>) -> Self {
        Self { vpns_repository }
    }

    pub async fn search_all_vpns(&self) -> Result<Option<Vec<VpnOutline>>, anyhow::Error> {
        debug!("services: search_all_vpns");
        match self.vpns_repository.find_all().await {
            Ok(vpn_outlines) => Ok(Some(vpn_outlines)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
