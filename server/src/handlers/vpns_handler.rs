use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

use log::debug;

use crate::{
    config::AppState,
    entities::{
        clients::{ClientCreateDto, ClientOutline},
        errors::AppError,
        responses::HttpResponse,
        terminals::TerminalOutline,
    },
};

//vpns
// /vpns
pub async fn search_all_vpns(State(state): State<AppState>) -> impl IntoResponse {
    debug!("handler: search_all_vpns");

    let result = state.vpns_service.search_all_vpns().await;
    match result {
        Ok(data) => HttpResponse::Ok(data).into_response(),
        Err(err) => err.into_response(),
    }
}

//servers
// /servers
pub async fn search_all_servers(State(state): State<AppState>) -> impl IntoResponse {
    debug!("handler search_all_servers");

    let result = state.server_service.search_all_servers().await;
    match result {
        Ok(data) => HttpResponse::Ok(data).into_response(),
        Err(err) => err.into_response(),
    }
}

//clients
// /clients?vpn_id=:vpn_id
pub async fn search_clients(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    debug!("handler search_clients");

    let result = state
        .clients_service
        .search_clients(query.get("vpn_id").unwrap())
        .await;

    match result {
        Ok(data) => HttpResponse::Ok(data).into_response(),
        Err(error) => error.into_response(),
    }
}

// post
// /clients
pub async fn create_clients(
    State(state): State<AppState>,
    Json(payload): Json<ClientCreateDto>,
) -> impl IntoResponse {
    debug!("handler create_clients");
    debug!("{:?}", payload);

    let mut client_outline: ClientOutline = (&payload.client_info).into();
    let terminal_info = payload.terminal_info;

    // terminal check
    // 新規端末登録あり
    if let Some(terminal_outline) = terminal_info {
        let terminal_outline: TerminalOutline = (&terminal_outline).into();
        let exists_terminal = state
            .terminals_service
            .exists(terminal_outline.terminal_id)
            .await;

        if !exists_terminal {
            let new_terminal_id = terminal_outline.terminal_id;

            let result = state.terminals_service.register(terminal_outline).await;
            if result.is_ok() {
                client_outline.set_terminal_id(new_terminal_id);
            } else if let Err(err) = result {
                return err.into_response();
            }
        } else {
            return AppError::InvalidInput(terminal_outline.terminal_id.to_string())
                .into_response();
        }

    // 新規端末登録なしで存在しない
    } else {
        let exists_terminal = state
            .terminals_service
            .exists(client_outline.terminal_id)
            .await;

        if !exists_terminal {
            return AppError::InvalidInput(client_outline.terminal_id.to_string()).into_response();
        }
    }

    // create clients
    let result = state
        .clients_service
        .register_client(client_outline, state.clone().config.aws_queue_url.clone())
        .await;

    match result {
        Ok(_) => HttpResponse::Created(()).into_response(),
        Err(err) => err.into_response(),
    }
}

// terminals
// /terminals?owner_user_id=:owner_user_id
pub async fn search_terminals(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    debug!("handler search_terminals");

    //validation
    let owner_user_id = query.get("owner_user_id");
    let owner_user_id = if let Some(owner_user_id) = owner_user_id {
        owner_user_id
    } else {
        return AppError::NotFound.into_response();
    };

    let result = state
        .terminals_service
        .search_by_owner_user_id(owner_user_id)
        .await;
    match result {
        Ok(data) => HttpResponse::Ok(data).into_response(),
        Err(err) => err.into_response(),
    }
}
