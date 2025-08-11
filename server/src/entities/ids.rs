use base64::{Engine, prelude::BASE64_STANDARD_NO_PAD};
use serde::{Deserialize, Serialize};

use crate::entities::errors::AppError;

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Debug)]
#[sqlx(transparent)]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    pub fn new(value: uuid::Uuid) -> Self {
        EntityId(value)
    }
}

impl TryFrom<String> for EntityId {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // decode
        let decoded_value = BASE64_STANDARD_NO_PAD
            .decode(value.clone())
            .map_err(|_| AppError::InvalidInput(value.clone()))?;

        // convert to UUID
        let id = uuid::Uuid::try_from(decoded_value)
            .map_err(|_| AppError::InvalidInput(value.clone()))?;

        Ok(EntityId(id))
    }
}

impl From<EntityId> for uuid::Uuid {
    fn from(value: EntityId) -> Self {
        value.0
    }
}
