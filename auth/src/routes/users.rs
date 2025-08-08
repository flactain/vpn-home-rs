use axum::{Router, routing::get};

use crate::handlers::users::list_users;

pub fn router() -> Router {
    Router::new().route("/{id}", get(list_users))
}
