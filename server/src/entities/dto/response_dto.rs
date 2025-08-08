use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ResponseDto {
    pub message: String,
    pub data: Value,
}
