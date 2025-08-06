use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::debug;
use serde_json::json;

use crate::{
    config::AppState,
    entities::{
        clients::{ClientOutline, ClientOutlineDto},
        dto::response_dto::ResponseDto,
    },
};

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
                //エラーは起こってないが、データがない場合は404で返却
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

pub async fn create_clients(
    State(state): State<AppState>,
    Json(payload): Json<ClientOutlineDto>,
) -> impl IntoResponse {
    debug!("handler create_clients");

    let client_outline: ClientOutline = (&payload).into();

    let (status_code, data, message) = match state
        .clone()
        .clients_service
        .register_client(client_outline)
        .await
    {
        Ok(_) => (StatusCode::CREATED, json!(""), "created".to_string()),
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
