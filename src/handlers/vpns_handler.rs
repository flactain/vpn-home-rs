use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use log::debug;
use serde_json::json;

use crate::{config::AppState, entities::dto::response_dto::ResponseDto};

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
