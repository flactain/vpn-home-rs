use serde::{Deserialize, Serialize};
use vpn_libs::entities::{
    ids::EntityId,
    messages::{MessageType, ResourceHandle, ResourceType},
};

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

impl From<ApprovalRequest> for MessageType {
    fn from(value: ApprovalRequest) -> Self {
        let resource_type = value.resource_type;
        let resource_handle = value.resource_handle;
        MessageType::new(resource_type, resource_handle)
    }
}
