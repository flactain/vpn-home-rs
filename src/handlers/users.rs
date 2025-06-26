use axum::Json;
use serde_json::{Value, json};

pub async fn list_users() -> Json<Value> {
    Json(json!({"data":42}))
}
