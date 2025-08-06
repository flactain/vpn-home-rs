use std::sync::Arc;


use crate::{
    entities::terminals::TerminalOutline,
    repositories::terminals_repository::TerminalsRepository,
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

    pub async fn create_if_not_exists(
        &self,
        terminal_info: TerminalOutline,
    ) -> Result<(), anyhow::Error> {
        if self
            .terminals_repository
            .exists_by_id(terminal_info.terminal_id)
            .await
            .unwrap()
        {
            Ok(())
        } else {
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
}
