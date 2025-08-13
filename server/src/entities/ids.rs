use std::fmt;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::{ContextV7, Uuid};

use crate::entities::errors::AppError;

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
#[derive(Default)]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    pub fn new() -> Self {
        EntityId(uuid::Uuid::new_v7(uuid::Timestamp::now(ContextV7::new())))
    }
}

impl From<EntityId> for String {
    fn from(val: EntityId) -> Self {
        BASE64_URL_SAFE_NO_PAD.encode(val.0)
    }
}

impl From<Uuid> for EntityId {
    fn from(value: Uuid) -> Self {
        EntityId(value)
    }
}

impl From<EntityId> for uuid::Uuid {
    fn from(value: EntityId) -> Self {
        value.0
    }
}

impl TryFrom<String> for EntityId {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        debug!("try from string");

        if value == String::default() {
            Ok(EntityId::default())
        } else {
            // decode
            let decoded_value = BASE64_URL_SAFE_NO_PAD
                .decode(value.clone())
                .map_err(|_| AppError::InvalidInput(value.clone()))?;

            // convert to UUID
            let id = uuid::Uuid::try_from(decoded_value)
                .map_err(|_| AppError::InvalidInput(value.clone()))?;

            Ok(EntityId(id))
        }
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
