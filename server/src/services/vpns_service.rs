use std::sync::Arc;

use log::debug;

use crate::{entities::{errors::AppError, vpns::VpnOutlineDto}, repositories::vpns_repository::VpnsRepository};

pub struct VpnsService {
    vpns_repository: Arc<dyn VpnsRepository>,
}

impl VpnsService {
    pub fn new(vpns_repository: Arc<dyn VpnsRepository>) -> Self {
        Self { vpns_repository }
    }

    pub async fn search_all_vpns(&self) -> Result<Vec<VpnOutlineDto>, AppError> {
        debug!("services: search_all_vpns");
        match self.vpns_repository.find_all().await {
            Ok(vpn_outlines) => Ok(vpn_outlines.iter().map(VpnOutlineDto::from).collect()),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(err.into()),
        }
    }
}
