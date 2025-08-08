use ::serde::Serialize;
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use sqlx::types::{chrono, ipnet::IpNet, uuid::Uuid};

#[derive(sqlx::FromRow, Serialize)]
pub struct ServerOutline {
    pub vpn_id: Uuid,
    pub vpn_name: String,
    pub terminal_id: Uuid,
    pub terminal_name: String,
    pub owner_user_id: String,
    pub public_ip: IpNet,
    pub private_ip: IpNet,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_approved: Option<bool>,
}

#[derive(Serialize)]
pub struct ServerOutlineDto {
    pub vpn_id: String,
    pub vpn_name: String,
    pub terminal_id: String,
    pub terminal_name: String,
    pub owner_user_id: String,
    pub public_ip: IpNet,
    pub private_ip: IpNet,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_approved: Option<bool>,
}

impl From<&ServerOutline> for ServerOutlineDto {
    fn from(server_outline: &ServerOutline) -> Self {
        ServerOutlineDto {
            vpn_id: BASE64_URL_SAFE_NO_PAD.encode(server_outline.vpn_id),
            vpn_name: server_outline.vpn_name.clone(),
            terminal_id: BASE64_URL_SAFE_NO_PAD.encode(server_outline.terminal_id),
            terminal_name: server_outline.terminal_name.clone(),
            owner_user_id: server_outline.owner_user_id.clone(),
            public_ip: server_outline.public_ip,
            private_ip: server_outline.private_ip,
            created_at: server_outline.created_at,
            is_approved: server_outline.is_approved,
        }
    }
}
