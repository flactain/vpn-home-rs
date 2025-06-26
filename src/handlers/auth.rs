use axum::{
    extract::{Path, State},
    response::{Redirect, Response},
};
use log::info;
use openidconnect::HttpResponse;

use crate::{config::AppState, dto::auth::CallbackParams, services::auth::AuthService};

pub async fn login(State(state): State<AppState>) -> Redirect {
    // init service
    let auth_service = AuthService::new(state.clone());

    // generate resource server auth url
    let (auth_url, _csrf, _nonce) = auth_service.resource_auth_url().await.unwrap();
    info!("generated url:{}", auth_url);

    Redirect::to(auth_url.as_str())
}

//http://127.0.0.1:8080/realms/local/broker/github/endpoint?code=720668456b50455b6e7a&state=OlDihfNaJoXAcSojfPIxzXpnYbzC4GKDBzVnCM6bpgg.V5lJG6KKyIE.asTBZsOrRV6v3TzR2HXByA
pub async fn callback(
    State(app_state): State<AppState>,
    Path(CallbackParams { code, state }): Path<CallbackParams>,
) {
    info!("get callback {},{}", code, state);
}
