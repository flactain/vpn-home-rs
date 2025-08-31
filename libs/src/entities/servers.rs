use ::serde::Serialize;
use serde::Deserialize;
use sqlx::types::{chrono, ipnet::IpNet};

use crate::entities::ids::EntityId;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Server {
    pub vpn_id: EntityId,
    pub vpn_name: String,
    pub terminal_id: EntityId,
    pub terminal_name: String,
    pub owner_user_id: String,
    pub public_ip: IpNet,
    pub private_ip: IpNet,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_approved: Option<bool>,
}
