use std::sync::Arc;

use openidconnect::{
    ClientId, ClientSecret, CsrfToken, EmptyAdditionalClaims, EmptyExtraTokenFields,
    EndpointMaybeSet, EndpointNotSet, EndpointSet, IdTokenFields, IssuerUrl, Nonce, RedirectUrl,
    RevocationErrorResponseType, StandardErrorResponse, StandardTokenIntrospectionResponse,
    StandardTokenResponse,
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreClient, CoreErrorResponseType,
        CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreRevocableToken, CoreTokenType,
    },
    reqwest::{self, Client},
    url::Url,
};
use tokio::sync::OnceCell;

use crate::config::{AppState, Config};
type DiscoveredClient = openidconnect::Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    StandardTokenResponse<
        IdTokenFields<
            EmptyAdditionalClaims,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
        >,
        CoreTokenType,
    >,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, CoreTokenType>,
    CoreRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;
/// Service for Authorization.
/// this service offers authorization processing based on OIDC.
/// you need to configure authorization server url, your be url.
/// 
/// # Examples
/// 
/// ```
/// let auth_service = AuthService::new();
/// let auth_client = auth_service.get_auth_client();
/// 
/// let (resource_auth_url, _csrf_token, _nonce) = auth_client.resource_auth_url();
/// ```
pub struct AuthService {
    /// cahce for authorization server client
    auth_client: OnceCell<DiscoveredClient>,
    /// cache for http Client
    http_client: Client,
    /// Config
    config: Arc<Config>,
}

impl AuthService {
    pub fn new(app_state: AppState) -> Self {
        AuthService {
            http_client: reqwest::ClientBuilder::new()
                // Following redirects opens the client up to SSRF vulnerabilities.
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("Cliend Cannot Build"),
            auth_client: OnceCell::new(),
            config: app_state.config.clone(),
        }
    }

    pub fn get_http_client(&self)-> &Client {
        &self.http_client
    }

    pub async fn get_auth_client(&self) -> anyhow::Result<&DiscoveredClient> {
        self.auth_client
            .get_or_try_init(|| async {
                let provider_metadata = CoreProviderMetadata::discover_async(
                    IssuerUrl::new(self.config.keycloak_url.clone()).unwrap(),
                    &self.http_client,
                )
                .await?;

                let client = CoreClient::from_provider_metadata(
                    provider_metadata,
                    ClientId::new(self.config.oauth_client_id.clone()),
                    Some(ClientSecret::new(self.config.oauth_client_secret.clone())),
                )
                .set_redirect_uri(RedirectUrl::new(format!(
                    "{}/auth/callback", //callbackapi
                    self.config.be_app_url_api.clone()
                ))?);

                Ok(client)
            })
            .await
    }

    pub async fn resource_auth_url(self) -> anyhow::Result<(Url, CsrfToken, Nonce)> {
        let (auth_url, csrf_token, nonce) = self
            .get_auth_client()
            .await?
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .url();
        Ok((auth_url, csrf_token, nonce))
    }
}
