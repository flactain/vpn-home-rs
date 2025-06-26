use axum::{extract::State, response::Redirect};
use log::info;

use crate::{config::AppState, services::auth::AuthService};

pub async fn login(State(state): State<AppState>) -> Redirect{
    // init service
    let auth_service = AuthService::new(state.clone());
    
    // generate resource server auth url
    let (auth_url, _csrf, _nonce) = auth_service.resource_auth_url().await.unwrap();
    info!("generated url:{}", auth_url);
    
    Redirect::to(auth_url.as_str())
}
