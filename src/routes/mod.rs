use axum::Router;

use crate::config::AppState;

pub mod auth;
pub mod peers;
pub mod users;

pub fn routers() -> Router<AppState>{
    Router::new()
        // .nest("/api/v1/users", users::router())
        // .nest("/api/v1/peers", peers::router())
        .nest("/api/v1/auth", auth::router())
}
