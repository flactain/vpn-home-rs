use ::serde::Serialize;
use sqlx::types::{chrono, ipnet::IpNet, uuid::Uuid};

#[derive(sqlx::FromRow, Serialize)]
pub struct ServerOutline {
    pub vpn_id: Uuid,
    pub vpn_name: String,
    pub terminal_id: Uuid,
    pub terminal_name: String,
    pub owner_user_id: String,
    pub public_ip: IpNet,
    pub created_at: Option<chrono::NaiveDateTime>,
}
