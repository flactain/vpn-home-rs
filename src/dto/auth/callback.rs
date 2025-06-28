
use serde::Deserialize;
#[derive(Deserialize)]
pub struct CallbackParams{
    pub code: String,
    pub state: String,
    pub session_state: String,
}
