use jsonwebtoken::{EncodingKey, Header};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use axum_session::{Session, SessionNullPool};
use openidconnect::{
    AccessToken, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyAdditionalClaims,
    EmptyExtraTokenFields, EndpointMaybeSet, EndpointNotSet, EndpointSet, IdTokenClaims,
    IdTokenFields, IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, RefreshToken, RevocationErrorResponseType, StandardErrorResponse,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenResponse,
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreClient, CoreErrorResponseType,
        CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreRevocableToken, CoreTokenType,
    },
    reqwest::{self, Client},
    url::Url,
};
use tokio::sync::OnceCell;

use crate::{
    config::{AppState, Config},
    dto::auth::SessionState,
};
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
/// this service offers authorization processing based on OIDC
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomClaims {
    pub user_id: String,
    pub name: String,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
    pub auth_time: Option<i64>,
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

    pub fn get_http_client(&self) -> &Client {
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

                warn!(
                    "issue url : {:?}",
                    IssuerUrl::new(self.config.keycloak_url.clone()).unwrap()
                );

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

    pub async fn resource_auth_url(
        &self,
    ) -> anyhow::Result<(Url, CsrfToken, Nonce, PkceCodeVerifier)> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token, nonce) = &self
            .get_auth_client()
            .await?
            .clone()
            .set_redirect_uri(RedirectUrl::new(format!(
                "{}/auth/callback", //callbackapi
                self.config.be_app_url_api.clone()
            ))?)
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .set_pkce_challenge(pkce_challenge)
            .url();
        Ok((
            auth_url.clone(),
            csrf_token.clone(),
            nonce.clone(),
            pkce_verifier,
        ))
    }

    pub async fn logout(&self) -> anyhow::Result<()> {
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(self.config.keycloak_url.clone()).unwrap(),
            &self.http_client,
        )
        .await?;

        Ok(())
    }

    pub async fn try_token(
        &self,
        session: Session<SessionNullPool>,
        state: String,
        code: String,
        session_state_id: &str,
    ) -> anyhow::Result<(String, AccessToken, RefreshToken)> {
        //get session state
        let session_value: String = session
            .get(session_state_id)
            .ok_or_else(|| anyhow::anyhow!("cannot get state in session store"))?;
        let session_state = serde_json::from_str::<SessionState>(&session_value)
            .expect("cannot parse state in store");

        //validate csrf token
        if state != session_state.csrf_token.into_secret() {
            anyhow::anyhow!("Different csrf token");
        }

        //get token
        let token = &self
            .get_auth_client()
            .await?
            .exchange_code(AuthorizationCode::new(code))?
            .set_pkce_verifier(session_state.pkce_verifier)
            .request_async(self.get_http_client())
            .await?;

        //get each token
        let id_token = token
            .id_token()
            .ok_or_else(|| anyhow::anyhow!("Cannot get ID token"))?;
        let access_token = token.access_token();
        let refresh_token = token.refresh_token().unwrap();

        debug!("idToken: {:?}", id_token.to_string());
        debug!("accessToken: {:?}", access_token);
        debug!("refreshToken:{:?}", refresh_token);

        Ok((
            id_token.to_string(),
            access_token.clone(),
            refresh_token.clone(),
        ))
    }

    fn convert(claims: &IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>) -> CustomClaims {
        CustomClaims {
            user_id: claims.preferred_username().unwrap().to_string(),
            name: claims.name().unwrap().get(None).unwrap().to_string(),
            iss: "vpn-home-rs".to_string(),
            exp: claims.expiration().timestamp(),
            iat: claims.issue_time().timestamp(),
            auth_time: Some(claims.auth_time().unwrap().timestamp()),
        }
    }

    pub fn make_jwt(
        &self,
        claims: IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>,
    ) -> anyhow::Result<String> {
        let custom_claims = Self::convert(&claims);
        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &custom_claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )?;
        Ok(jwt)
    }
}
