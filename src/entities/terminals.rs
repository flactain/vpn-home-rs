use serde::Serialize;
use sqlx::types::{chrono, uuid::Uuid};

#[derive(sqlx::FromRow, Serialize)]
pub struct TerminalOutline {
    terminal_id: Uuid,
    terminal_name: String,
    os: Option<String>,
    is_deleted: bool,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct TerminalOutlineDto {
    terminal_id: String,
    terminal_name: String,
    os: Option<String>,
    is_deleted: bool,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}
