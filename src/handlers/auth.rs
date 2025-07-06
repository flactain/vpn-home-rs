use std::{collections::HashMap, str::FromStr};

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue},
    response::{IntoResponse, Redirect, Response},
};
use axum_cookie::{
    CookieManager,
    cookie::{Cookie, CookieJar},
};
use axum_session::{Session, SessionNullPool};
use chrono::format;
use log::{debug, error, info};
use openidconnect::{HttpRequest, PkceCodeVerifier, url::Url};
use uuid::Uuid;

use crate::{
    config::AppState,
    dto::auth::{CallbackParams, SessionState},
    services::auth::AuthService,
};

///  login handler
///
/// # Examples
///
/// ```
/// example code
/// ```
pub async fn login(
    State(state): State<AppState>,
    session: Session<SessionNullPool>,
    cookie_manager: CookieManager,
) -> impl IntoResponse {
    // init service
    let auth_service = AuthService::new(state.clone());

    // generate resource server auth url
    let (auth_url, csrf, nonce, pkce_verifier) = auth_service.resource_auth_url().await.unwrap();
    info!("generated url:{}", auth_url);

    // create session state for verifying in callback handler
    let session_state_id = Uuid::new_v4().to_string();
    let session_state = SessionState::new(
        csrf,
        nonce,
        pkce_verifier,
        (chrono::Utc::now() + chrono::Duration::minutes(10)).timestamp(),
    );
    let session_state = serde_json::to_string(&session_state).unwrap().clone();

    debug!("session_state_id: {}", session_state_id.to_string());
    debug!("session_state_value: {}", session_state.to_string());
    session.set(session_state_id.as_str(), session_state);

    //add session_state_id to Cookie.
    let mut cookie = Cookie::new("session_state_id", session_state_id);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie_manager.add(cookie);

    Redirect::to(auth_url.as_str()).into_response()
}
pub async fn logout(State(state): State<AppState>) -> impl IntoResponse {
    //init service
    Redirect::to(
                format!("http://127.0.0.1:8080/realms/local/protocol/openid-connect/logout?id_token_hint={}&post_logout_redirect_uri={}",
                "a",state.config.fe_app_url.clone()).as_str()
                )
                .into_response()
}

/// login callback called by keycloak(idp: github)
///
/// # Examples
///
/// ```
/// example code
/// ```
pub async fn callback(
    State(app_state): State<AppState>,
    session: Session<SessionNullPool>,
    cookie_manager: CookieManager,
    callback_params: Query<CallbackParams>,
) -> impl IntoResponse {
    debug!("callback!!");
    //init service
    let auth_service = AuthService::new(app_state.clone());

    debug!("parameters:{:?}", callback_params);
    debug!(
        "cookie:session_state_id: {}",
        cookie_manager.get("session_state_id").unwrap()
    );

    // get claims
    let (id_token, access_token, refresh_token) = auth_service
        .try_token(
            session,
            callback_params.state.clone(),
            callback_params.code.clone(),
            cookie_manager.get("session_state_id").unwrap().value(),
        )
        .await
        .unwrap();

    //remove cookie
    cookie_manager.remove("session_state_id");

    // set refresh_token to cookie
    let mut cookie = Cookie::new("refresh_token", refresh_token.into_secret());
    cookie.set_path("/");
    cookie.set_http_only(false);
    cookie_manager.set(cookie);

    // set fragment to redirect url
   let mut redirect_url = Url::parse(format!("{}/callback", app_state.config.fe_app_url.clone()).as_str()).unwrap();
        redirect_url.set_fragment(Some(&format!("id_token={}", id_token)));
        redirect_url.set_fragment(Some(&format!("access_token={}", access_token.into_secret())));

    (Redirect::to(redirect_url.as_str()).into_response(),)
}
