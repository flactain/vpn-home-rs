use axum::Router;

pub mod peers;
pub mod users;

pub fn routers() -> Router {
    Router::new()
        .nest("/api/v1/users", users::router())
        .nest("/api/v1/peers", peers::router())
}
