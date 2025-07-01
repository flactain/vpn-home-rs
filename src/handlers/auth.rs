use std::str::FromStr;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue},
    response::{IntoResponse, Redirect, Response},
};
use axum_session::{Session, SessionNullPool};
use jsonwebtoken::Header;
use log::{debug, info};
use serde::Serialize;

use crate::{config::AppState, dto::auth::callback::CallbackParams, services::auth::AuthService};

///  login handler
/// 
/// # Examples
/// 
/// ```
/// example code
/// ```
pub async fn login(State(state): State<AppState>, session: Session<SessionNullPool>) -> Redirect {
    // init service
    let auth_service = AuthService::new(state.clone());

    // generate resource server auth url
    let (auth_url, csrf, nonce) = auth_service.resource_auth_url().await.unwrap();
    info!("generated url:{}", auth_url);

    // register session on cookie
    session.set("csrf_token", csrf);
    session.set("nonce", nonce);

    Redirect::to(auth_url.as_str())
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
    callback_params: Query<CallbackParams>,
) -> impl IntoResponse {

    debug!("callback!!");
    //init service
    let auth_service = AuthService::new(app_state.clone());

    // get claims
    let Ok(claims) = auth_service
        .try_token(
            session,
            callback_params.state.clone(),
            callback_params.code.clone(),
        )
        .await
    else {
        return (
            None,
            Redirect::to(format!("{}/", app_state.config.be_app_url).as_str()).into_response(),
        );
    };

    //set jwt to headers.
    let jwt = auth_service.make_jwt(claims).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(format!("Bearer {}", jwt).as_str()).unwrap(),
    );
    headers.insert(
        "Set-Cookie",
        HeaderValue::from_str(format!("jwt={}; HttpOnly:false; Path=/", jwt).as_str()).unwrap(),
    );

    (
        Some(headers),
        Redirect::to(format!("{}/home", app_state.config.fe_app_url.clone()).as_str())
            .into_response(),
    )
}
