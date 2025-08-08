use axum::extract::OriginalUri;
use log::warn;

pub async fn fallback_handler(OriginalUri(uri): OriginalUri) -> &'static str {
    warn!("fallback{}", uri.path());
    "NOT FOUND"
}
