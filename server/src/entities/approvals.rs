use serde::{Deserialize, Serialize};
use vpn_libs::entities::ids::EntityId;

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Debug)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceType {
    Vpn,
    Client,
}

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Debug)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceHandle {
    Create,
    Edit,
    Delete,
    Archive,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Debug)]
pub struct ApprovalRequest {
    pub resource_type: ResourceType,
    pub resource_handle: ResourceHandle,
    pub vpn_id: EntityId,
    pub resource_id: EntityId,
    #[serde(default)]
    pub asignee_user_id: String,
    #[serde(default)]
    pub request_user_id: String,
    pub approved_at: Option<chrono::NaiveDateTime>,
}
