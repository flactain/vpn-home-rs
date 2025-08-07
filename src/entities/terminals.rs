use base64::{
    Engine,
    prelude::{BASE64_STANDARD_NO_PAD, BASE64_URL_SAFE_NO_PAD},
};
use serde::{Deserialize, Serialize};
use sqlx::types::{chrono, uuid::Uuid};
use uuid::ContextV7;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct TerminalOutline {
    pub terminal_id: Uuid,
    pub terminal_name: String,
    pub owner_user_id: String,
    pub os: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl TerminalOutline {}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct TerminalOutlineDto {
    pub terminal_id: String,
    pub terminal_name: Option<String>,
    pub owner_user_id: String,
    pub os: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

// Convert
impl From<&TerminalOutlineDto> for TerminalOutline {
    fn from(terminal_outline_dto: &TerminalOutlineDto) -> Self {
        TerminalOutline {
            terminal_id: match terminal_outline_dto.terminal_id.as_str() {
                // empty => new, else decode
                "" => uuid::Uuid::new_v7(uuid::Timestamp::now(ContextV7::new())),
                b64_uuid => {
                    Uuid::try_from(BASE64_STANDARD_NO_PAD.decode(b64_uuid).unwrap()).unwrap()
                }
            },
            terminal_name: match terminal_outline_dto.terminal_name.clone() {
                Some(terminal_name) => terminal_name.clone(),
                None => "".to_string(),
            },
            owner_user_id: terminal_outline_dto.owner_user_id.clone(),
            os: terminal_outline_dto.os.clone(),
            created_at: terminal_outline_dto.created_at,
            updated_at: terminal_outline_dto.updated_at,
        }
    }
}

impl From<&TerminalOutline> for TerminalOutlineDto {
    fn from(terminal_outline: &TerminalOutline) -> Self {
        TerminalOutlineDto {
            terminal_id: BASE64_URL_SAFE_NO_PAD.encode(terminal_outline.terminal_id),
            terminal_name: Some(terminal_outline.terminal_name.clone()),
            owner_user_id: terminal_outline.owner_user_id.clone(),
            os: terminal_outline.os.clone(),
            created_at: terminal_outline.created_at,
            updated_at: terminal_outline.updated_at,
        }
    }
}
