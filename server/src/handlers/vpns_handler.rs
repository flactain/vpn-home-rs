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
        approvals::{ApprovalRequest, ResourceType},
        errors::AppError,
        ids::EntityId,
        responses::HttpResponse,
    },
    handlers::dto::client_create_dto::ClientCreateDto,
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

pub async fn search_requests(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    debug!("handler: search_request");

    // extract
    let Some(user_id) = query.get("user_id") else {
        return AppError::InvalidInput("user_id".to_string()).into_response();
    };

    // data fetch
    let result = state.vpns_service.search_requests(user_id).await;

    match result {
        Ok(approval_requests) => HttpResponse::Ok(approval_requests).into_response(),
        Err(err) => err.into_response(),
    }
}

pub async fn approve_request(
    State(state): State<AppState>,
    Json(payload): Json<ApprovalRequest>,
) -> impl IntoResponse {
    debug!("{:?}", payload);

    let result = match payload.resource_type() {
        ResourceType::Vpn => state.vpns_service.approve_vpn(payload).await,
        ResourceType::Client => state.vpns_service.approve_client(payload).await,
    };

    match result {
        Ok(_) => HttpResponse::<String>::Updated.into_response(),
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

    // extract
    let Some(vpn_id) = query.get("vpn_id") else {
        return AppError::InvalidInput("vpn_id".to_string()).into_response();
    };

    // transform
    let vpn_id = match EntityId::try_from(vpn_id.to_owned()) {
        Ok(vpn_id) => vpn_id,
        Err(err) => return err.into_response(),
    };

    // data fetch
    let result = state.clients_service.search_clients(vpn_id).await;

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

    let mut client_outline = payload.client_info;
    let terminal_outline = payload.terminal_info;

    let mut tx = state.pool.begin().await.unwrap();
    // terminal check
    // 新規端末登録あり
    if let Some(mut terminal_outline) = terminal_outline {
        if terminal_outline.terminal_id == uuid::Uuid::default().into() {
            terminal_outline.terminal_id = EntityId::new();

            let result = state
                .terminals_service
                .register(&mut tx, &terminal_outline)
                .await;

            if result.is_ok() {
                client_outline.terminal_id = terminal_outline.terminal_id;
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
            .exists(&client_outline.terminal_id)
            .await;

        if !exists_terminal {
            return AppError::InvalidInput(client_outline.terminal_id.to_string()).into_response();
        }
    }

    // create clients
    let result = state
        .clients_service
        .register_client(&mut tx, client_outline)
        .await;

    tx.commit().await.unwrap();

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
        return AppError::InvalidInput("owner_user_id".to_string()).into_response();
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
