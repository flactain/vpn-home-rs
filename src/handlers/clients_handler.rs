use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::debug;
use serde_json::json;

use crate::{config::AppState, entities::dto::response_dto::ResponseDto};

pub async fn search_clients(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    debug!("handler search_clients");

    let (status_code, data, message) = match state
        .clone()
        .clients_service
        .search_clients(query.get("vpn_id").unwrap())
        .await
    {
        Ok(clients_outline) => {
            if let Some(clients_outline) = clients_outline {
                (
                    StatusCode::OK,
                    json!(clients_outline),
                    "success".to_string(),
                )
            } else {
                (StatusCode::NOT_FOUND, json!(""), "no data".to_string())
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!(""),
            err.to_string(),
        ),
    };

    (
        status_code,
        Json(ResponseDto {
            message: message.to_string(),
            data,
        }),
    )
}
