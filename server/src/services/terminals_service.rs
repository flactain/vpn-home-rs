use std::sync::Arc;

use log::debug;
use sqlx::Transaction;
use vpn_libs::entities::{errors::AppError, ids::EntityId, terminals::TerminalOutline};

use crate::repositories::terminals_repository::TerminalsRepository;

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
    ) -> Result<Vec<TerminalOutline>, AppError> {
        debug!("services: search_by_owner_user_id");

        match self
            .terminals_repository
            .find_by_user_id(owner_user_id)
            .await
        {
            Ok(server_outlines) => Ok(server_outlines),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn exists(&self, terminal_id: &EntityId) -> bool {
        self.terminals_repository
            .exists_by_id(terminal_id)
            .await
            .is_ok()
    }

    pub async fn register(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        terminal_info: &TerminalOutline,
    ) -> Result<(), AppError> {
        match self.terminals_repository.create(tx, terminal_info).await {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("cannot register terminal.").into())
                }
            }
            Err(err) => Err(AppError::DatabaseError(err)),
        }
    }
}
