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
        clients::{ClientCreate, ClientCreateDto},
        dto::response_dto::ResponseDto,
    },
};

//vpns
pub async fn search_all_vpns(State(state): State<AppState>) -> impl IntoResponse {
    debug!("handler: search_all_vpns");
    let vpns_service = state.clone().vpns_service.clone();
    let (status_code, data, message) = match vpns_service.search_all_vpns().await {
        Ok(Some(vpns)) => (StatusCode::OK, json!(vpns), "success"),
        Ok(None) => (StatusCode::NOT_FOUND, json!(""), "no data"),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!(""),
            "some error occured",
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

//servers
pub async fn search_all_servers(State(state): State<AppState>) -> impl IntoResponse {
    debug!("handler search_all_servers");
    let (status_code, data, message) = match state.clone().server_service.search_all_servers().await
    {
        Ok(servers) => {
            if let Some(servers) = servers {
                (StatusCode::OK, json!(servers), "success")
            } else {
                (StatusCode::NOT_FOUND, json!(""), "no data")
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!(""),
            "any error occured",
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

//clients
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
    Json(payload): Json<ClientCreateDto>,
) -> impl IntoResponse {
    debug!("handler create_clients");

    debug!("{:?}", payload);
    let client_info: ClientCreate = (&payload).into();

    // terminal check
    // 上の変換だとだめっぽいなあ

    // create clients
    let (status_code, data, message) = match state
        .clone()
        .clients_service
        .register_client(client_info)
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
