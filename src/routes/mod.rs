use axum::{response::IntoResponse, routing::get, Router};

use crate::config::AppState;

pub fn routers() ->Router<AppState> {
    Router::new().nest("/api/v1/vpn", vpnRouter())
}

fn vpnRouter() -> Router<AppState>{
    Router::new().route("servers", get(searchServers))
}

async fn searchServers() -> impl IntoResponse{
    
}
