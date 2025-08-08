use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use log::{debug, error};
use serde_json::json;

use crate::{
    config::AppState,
    entities::{
        clients::{ClientCreateDto, ClientOutline},
        dto::response_dto::ResponseDto,
        terminals::{TerminalOutline, TerminalOutlineDto},
    },
};

//vpns
// /vpns
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
// /servers
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
// /clients?vpn_id=:vpn_id
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

// post
// /clients
pub async fn create_clients(
    State(state): State<AppState>,
    Json(payload): Json<ClientCreateDto>,
) -> impl IntoResponse {
    debug!("handler create_clients");

    let terminals_service = state.clone().terminals_service;
    let clients_service = state.clone().clients_service;

    debug!("{:?}", payload);
    let client_info = payload.client_info;
    let mut client_outline: ClientOutline = (&client_info).into();
    let terminal_info = payload.terminal_info;

    // terminal check
    // 新規端末登録あり
    if let Some(terminal_outline) = terminal_info {
        let terminal_outline: TerminalOutline = (&terminal_outline).into();

        if !terminals_service.exists(terminal_outline.terminal_id).await {
            let new_terminal_id = terminal_outline.terminal_id;
            terminals_service.register(terminal_outline).await.unwrap();
            client_outline.set_terminal_id(new_terminal_id);
        } else {
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseDto {
                    message: "input terminal already exists".to_string(),
                    data: json!("".to_string()),
                }),
            );
        }
    // 新規端末登録なし
    } else if !terminals_service.exists(client_outline.terminal_id).await {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseDto {
                message: "input terminal not exists".to_string(),
                data: json!("".to_string()),
            }),
        );
    }

    // create clients
    let (status_code, data, message) = match clients_service
        .register_client(client_outline, state.clone().config.aws_queue_url.clone())
        .await
    {
        Ok(_) => (StatusCode::CREATED, json!(""), "created".to_string()),
        Err(err) => {
            error!("{}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(""),
                err.to_string(),
            )
        }
    };

    (
        status_code,
        Json(ResponseDto {
            message: message.to_string(),
            data,
        }),
    )
}

// terminals
// /terminals?owner_user_id=:owner_user_id
pub async fn search_terminals(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    //validation
    let owner_user_id = query.get("owner_user_id");
    let owner_user_id = if let Some(owner_user_id) = owner_user_id {
        owner_user_id
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseDto {
                message: "owner not found in parameter".to_string(),
                data: json!(""),
            }),
        );
    };

    let (status_code, data, message) = match state
        .clone()
        .terminals_service
        .search_by_owner_user_id(owner_user_id)
        .await
    {
        Ok(terminal_outlines) => {
            if let Some(terminal_outlines) = terminal_outlines {
                (
                    StatusCode::OK,
                    json!(
                        terminal_outlines
                            .iter()
                            .map(TerminalOutlineDto::from)
                            .collect::<Vec<_>>()
                    ),
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
