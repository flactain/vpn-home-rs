use ::serde::Serialize;
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use serde::Deserialize;
use sqlx::types::{chrono, ipnet::IpNet, uuid::Uuid};

#[derive(sqlx::FromRow, Serialize)]
pub struct ClientOutline {
    pub vpn_id: Uuid,
    pub vpn_name: String,
    pub terminal_id: Uuid,
    pub terminal_name: String,
    pub owner_user_id: String,
    pub allowed_ip: Option<IpNet>,
    pub public_key: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_approved: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientOutlineDto {
    pub vpn_id: String,
    pub vpn_name: String,
    pub terminal_id: String,
    pub terminal_name: String,
    pub owner_user_id: String,
    pub allowed_ip: Option<IpNet>,
    pub public_key: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_approved: Option<bool>,
}

impl From<&ClientOutline> for ClientOutlineDto {
    fn from(client_outline: &ClientOutline) -> Self {
        ClientOutlineDto {
            vpn_id: BASE64_URL_SAFE_NO_PAD.encode(client_outline.vpn_id),
            vpn_name: client_outline.vpn_name.clone(),
            terminal_id: BASE64_URL_SAFE_NO_PAD.encode(client_outline.terminal_id),
            terminal_name: client_outline.terminal_name.clone(),
            owner_user_id: client_outline.owner_user_id.clone(),
            allowed_ip: client_outline.allowed_ip,
            public_key: client_outline.public_key.clone(),
            created_at: client_outline.created_at,
            is_approved: client_outline.is_approved,
        }
    }
}

impl From<&ClientOutlineDto> for ClientOutline {
    fn from(client_outline_dto: &ClientOutlineDto) -> Self {
        ClientOutline {
            vpn_id: Uuid::try_from(
                BASE64_URL_SAFE_NO_PAD
                    .decode(client_outline_dto.vpn_id.clone())
                    .unwrap(),
            )
            .unwrap(),
            vpn_name: client_outline_dto.vpn_name.clone(),
            terminal_id: Uuid::try_from(
                BASE64_URL_SAFE_NO_PAD
                    .decode(client_outline_dto.terminal_id.clone())
                    .unwrap(),
            )
            .unwrap(),
            terminal_name: client_outline_dto.terminal_name.clone(),
            owner_user_id: client_outline_dto.owner_user_id.clone(),
            allowed_ip: client_outline_dto.allowed_ip,
            public_key: client_outline_dto.public_key.clone(),
            created_at: client_outline_dto.created_at,
            is_approved: client_outline_dto.is_approved,
        }
    }
}
