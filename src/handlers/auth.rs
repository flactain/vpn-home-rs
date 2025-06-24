use anyhow::Result;
use std::sync::Arc;

use axum::extract::State;
use log::info;
use openidconnect::{
    ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl,
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest,
};

use crate::config::AppState;

pub async fn login(State(state): State<AppState>) {
    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Cliend Cannot Build");

    //configure OpenID Connect Provider
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(state.config.keycloak_url.clone()).unwrap(),
        &http_client,
    )
    .await.unwrap();

    // configure client
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(state.config.oauth_client_id.clone()),
        Some(ClientSecret::new(state.config.oauth_client_secret.clone())),
    )
    .set_redirect_uri(RedirectUrl::new(format!(
        "{}/auth/callback", //callbackapi
        state.config.be_app_url_api.clone()
    )).unwrap());

    //generate request to authorization server
    let (auth_url, csrf_token, nonce) = client
 
 
      .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .url();
    info!("generated url:{}", auth_url);
}
