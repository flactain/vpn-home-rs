use axum::Router;

pub mod peers;
pub mod users;
pub mod auth;

pub fn routers() -> Router {
    Router::new()
        .nest("/api/v1/users", users::router())
        .nest("/api/v1/peers", peers::router())
        .nest("/api/v1/auth", auth::router())
}
