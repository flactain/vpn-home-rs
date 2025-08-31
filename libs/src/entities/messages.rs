use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MessageType {
    pub resource_type: ResourceType,
    pub resource_handle: ResourceHandle,
}

impl MessageType {
    pub fn new(resource_type: ResourceType, resource_handle: ResourceHandle) -> Self {
        Self {
            resource_type,
            resource_handle,
        }
    }
}

#[derive(sqlx::Type, Deserialize, Serialize, Clone, Debug)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceType {
    Vpn,
    Client,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResourceType::Vpn => "VPN",
            ResourceType::Client => "CLIENT",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for ResourceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResourceHandle::Create => "CREATE",
            ResourceHandle::Edit => "EDIT",
            ResourceHandle::Delete => "DELETE",
            ResourceHandle::Archive => "ARCHIVE",
            ResourceHandle::Approve => "APPROVE",
        };

        write!(f, "{}", s)
    }
}
#[derive(sqlx::Type, Deserialize, Serialize, Clone, Debug)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceHandle {
    Create,
    Edit,
    Delete,
    Archive,
    Approve,
}

#[repr(u32)]
pub enum MessagePriority {
    VeryHigh = 100,
    High = 80,
    Normal = 60,
    Low = 40,
    VeryLow = 20,
    Unnecesarry = 0,
}
