use openidconnect::{core::{CoreClient, CoreProviderMetadata}, reqwest, ClientId, IssuerUrl};

pub async fn login() {
    let http_client = reqwest::blocking::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Cliend Cannot Build");

    //configure OpenID Connect Provider
    let provider_metadata = CoreProviderMetadata::discover(
        &IssuerUrl::new("http://localhost:8080".to_string()).unwrap(),
        &http_client,
    ).unwrap();

    let client = CoreClient::from_provider_metadata(provider_metadata, ClientId::new("vpn-home"), client_secret);
}
