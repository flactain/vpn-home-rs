use ::serde::Serialize;
use sqlx::types::{chrono, ipnet::IpNet};

use crate::entities::ids::EntityId;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct VpnOutline {
    pub vpn_id: EntityId,
    pub vpn_name: String,
    pub owner_user_id: String,
    pub server_name: String,
    pub public_ip: IpNet,
    pub private_ip: IpNet,
    pub clients_count: Option<i64>,
    pub is_approved: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
