use axum::{extract::State, response::IntoResponse};

use crate::config::AppState;

pub async fn search_all_servers(State(state): State<AppState>) -> impl IntoResponse {}
