use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use log::debug;
use serde_json::json;

use crate::{
    config::AppState,
    entities::dto::response_dto::ResponseDto,
};

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
