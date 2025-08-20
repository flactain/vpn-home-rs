use ::serde::Serialize;
use serde::Deserialize;
use sqlx::types::{chrono, ipnet::IpNet};

use crate::entities::ids::EntityId;

// Client概要
#[derive(sqlx::FromRow, Serialize, Clone, Deserialize, Debug)]
pub struct ClientOutline {
    pub vpn_id: EntityId,
    pub vpn_name: Option<String>,
    pub terminal_id: EntityId,
    pub terminal_name: Option<String>,
    pub owner_user_id: Option<String>,
    pub allowed_ip: Option<IpNet>,
    pub public_key: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub is_approved: Option<bool>,
}
