use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    RequestVpn,
    RequestClient,
    ApproveClient,
    ApproveVpn,
    Default,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
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
