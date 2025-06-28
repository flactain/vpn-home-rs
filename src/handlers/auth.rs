use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_session::{Session, SessionNullPool};
use log::info;
use serde::Serialize;

use crate::{config::AppState, dto::auth::callback::CallbackParams, services::auth::AuthService};

pub async fn login(State(state): State<AppState>, session: Session<SessionNullPool>) -> Redirect {
    // init service
    let auth_service = AuthService::new(state.clone());

    // generate resource server auth url
    let (auth_url, csrf, _nonce) = auth_service.resource_auth_url().await.unwrap();
    info!("generated url:{}", auth_url);

    // register session on cookie
    session.set("oauth_state", csrf);

    Redirect::to(auth_url.as_str())
}

pub async fn callback(
    State(app_state): State<AppState>,
    session: Session<SessionNullPool>,
    callback_params: Query<CallbackParams>,
) -> Response {
    info!(
        "get callback {},{},{}",
        callback_params.code, callback_params.state, callback_params.session_state
    );

    let res_csrf = session.get("oauth_state").unwrap_or("failed".to_string());

    if callback_params.state != res_csrf {
        return Redirect::to(format!("{}/", app_state.config.be_app_url).as_str()).into_response();
    }

    Redirect::to(format!("{}/home", app_state.config.be_app_url).as_str()).into_response()
}
