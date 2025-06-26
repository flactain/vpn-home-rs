use crate::{config::AppState, handlers::auth};
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route("/login", get(auth::login))
    .route("/callback", get(auth::callback))
}

