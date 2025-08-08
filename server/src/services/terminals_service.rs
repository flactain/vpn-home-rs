use std::sync::Arc;

use log::debug;

use crate::{
    entities::terminals::TerminalOutline, repositories::terminals_repository::TerminalsRepository,
};

pub struct TerminalsService {
    terminals_repository: Arc<dyn TerminalsRepository>,
}

impl TerminalsService {
    pub fn new(terminals_repository: Arc<dyn TerminalsRepository>) -> Self {
        TerminalsService {
            terminals_repository,
        }
    }

    pub async fn search_by_owner_user_id(
        &self,
        owner_user_id: &str,
    ) -> Result<Option<Vec<TerminalOutline>>, anyhow::Error> {
        debug!("services: search_by_owner_user_id");

        match self
            .terminals_repository
            .find_by_user_id(owner_user_id)
            .await
        {
            Ok(server_outlines) => Ok(Some(server_outlines)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn exists(&self, terminal_id: uuid::Uuid) -> bool {
        self.terminals_repository
            .exists_by_id(terminal_id)
            .await
            .unwrap()
    }

    pub async fn register(&self, terminal_info: TerminalOutline) -> Result<(), anyhow::Error> {
        match self.terminals_repository.create(terminal_info).await {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("cannot register terminal."))
                }
            }
            Err(_) => Err(anyhow::anyhow!("something go wrong")),
        }
    }
}
