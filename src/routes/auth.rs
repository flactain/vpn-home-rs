use axum::{routing::get, Router};
use crate::handlers::auth;

pub fn router() -> Router{
    Router::new().route("/login", get(auth::login))
}
