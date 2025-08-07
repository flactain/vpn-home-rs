use ::serde::Serialize;
use base64::{
    Engine,
    prelude::{BASE64_STANDARD_NO_PAD, BASE64_URL_SAFE_NO_PAD},
};
use serde::Deserialize;
use sqlx::types::{chrono, ipnet::IpNet, uuid::Uuid};

use crate::entities::terminals::TerminalOutlineDto;

// Client概要
#[derive(sqlx::FromRow, Serialize)]
pub struct ClientOutline {
    pub vpn_id: Uuid,
    pub vpn_name: Option<String>,
    pub terminal_id: Uuid,
    pub terminal_name: Option<String>,
    pub owner_user_id: Option<String>,
    pub allowed_ip: Option<IpNet>,
    pub public_key: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_approved: Option<bool>,
}

impl ClientOutline {
    pub fn set_terminal_id(&mut self, terminal_id: Uuid) {
        self.terminal_id = terminal_id;
    }
}

// Client概要Dto
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientOutlineDto {
    pub vpn_id: String,
    pub vpn_name: Option<String>,
    pub terminal_id: String,
    pub terminal_name: Option<String>,
    pub owner_user_id: Option<String>,
    pub allowed_ip: Option<IpNet>,
    pub public_key: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_approved: Option<bool>,
}

//Client 作成Dto
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCreateDto {
    pub client_info: ClientOutlineDto,
    pub terminal_info: Option<TerminalOutlineDto>,
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
            terminal_id: match client_outline_dto.terminal_id.as_str() {
                // empty => new, else decode
                "" => uuid::Uuid::nil(),
                b64_uuid => {
                    Uuid::try_from(BASE64_STANDARD_NO_PAD.decode(b64_uuid).unwrap()).unwrap()
                }
            },
            terminal_name: client_outline_dto.terminal_name.clone(),
            owner_user_id: client_outline_dto.owner_user_id.clone(),
            allowed_ip: client_outline_dto.allowed_ip,
            public_key: client_outline_dto.public_key.clone(),
            created_at: client_outline_dto.created_at,
            is_approved: client_outline_dto.is_approved,
        }
    }
}
