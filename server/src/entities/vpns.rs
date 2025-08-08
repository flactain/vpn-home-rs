use ::serde::Serialize;
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use sqlx::types::{chrono, ipnet::IpNet, uuid::Uuid};

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct VpnOutline {
    pub vpn_id: Uuid,
    pub vpn_name: String,
    pub owner_user_id: String,
    pub server_name: String,
    pub public_ip: IpNet,
    pub private_ip: IpNet,
    pub clients_count: Option<i64>,
    pub is_approved: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
pub struct VpnOutlineDto {
    pub vpn_id: String,
    pub vpn_name: String,
    pub owner_user_id: String,
    pub server_name: String,
    pub public_ip: IpNet,
    pub private_ip: IpNet,
    pub clients_count: Option<i64>,
    pub is_approved: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<&VpnOutline> for VpnOutlineDto {
    fn from(value: &VpnOutline) -> Self {
        VpnOutlineDto {
            vpn_id: BASE64_URL_SAFE_NO_PAD.encode(value.vpn_id.as_bytes()),
            vpn_name: value.vpn_name.clone(),
            owner_user_id: value.owner_user_id.clone(),
            server_name: value.server_name.clone(),
            public_ip: value.public_ip,
            private_ip: value.private_ip,
            clients_count: value.clients_count,
            is_approved: value.is_approved,
            created_at: value.created_at,
        }
    }
}
