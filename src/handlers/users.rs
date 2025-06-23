use axum::Json;
use serde_json::{json, Value};

pub async fn list_users()-> Json<Value>{
    Json(json!({"data":42}))
}
