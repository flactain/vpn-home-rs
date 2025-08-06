use serde::{Deserialize, Serialize};
use sqlx::types::{chrono, uuid::Uuid};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct TerminalOutline {
    pub terminal_id: Uuid,
    pub terminal_name: String,
    pub os: Option<String>,
    pub is_deleted: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct TerminalOutlineDto {
    pub terminal_id: String,
    pub terminal_name: String,
    pub os: Option<String>,
    pub is_deleted: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
