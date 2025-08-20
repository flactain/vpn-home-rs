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
    resource_type: ResourceType,
    resource_handle: ResourceHandle,
    vpn_id: EntityId,
    resource_id: EntityId,
    #[serde(default)]
    asignee_user_id: String,
    #[serde(default)]
    request_user_id: String,
    approved_at: Option<chrono::NaiveDateTime>,
}

impl ApprovalRequest {
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type.clone()
    }
    pub fn vpn_id(&self) -> EntityId {
        self.vpn_id.clone()
    }
    pub fn resource_id(&self) -> EntityId {
        self.resource_id.clone()
    }
}
