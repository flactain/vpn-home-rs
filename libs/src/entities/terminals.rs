use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

use crate::entities::ids::EntityId;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct TerminalOutline {
    #[serde(default)]
    pub terminal_id: EntityId,
    pub terminal_name: String,
    pub owner_user_id: String,
    pub os: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
