use ::serde::Serialize;
use sqlx::types::{chrono, ipnet::IpNet, uuid::Uuid};

#[derive(sqlx::FromRow, Serialize)]
pub struct VpnOutline {
    pub vpn_id: Uuid,
    pub vpn_name: String,
    pub owner_user_id: String,
    pub server_name: String,
    pub public_ip: IpNet,
    pub clients_count: Option<i64>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
