use openidconnect::{CsrfToken, Nonce, PkceCodeVerifier};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct CallbackParams {
    pub code: String,
    pub state: String,
    pub session_state: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SessionState {
    pub csrf_token: CsrfToken,
    pub nonce: Nonce,
    pub pkce_verifier: PkceCodeVerifier,
    pub exp: i64, 
}

impl SessionState {
    pub fn new(
        csrf_token: CsrfToken,
        nonce: Nonce,
        pkce_verifier: PkceCodeVerifier,
        exp: i64 ,
    ) -> Self {
        SessionState {
            csrf_token,
            nonce,
            pkce_verifier,
            exp,
        }
    }
}
