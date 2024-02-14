use axum::{routing::get, Router};
pub fn health_route() -> Router {
    Router::new().route("/health/check", get(root))
}

pub async fn root() -> String {
    "Hello, World!".to_string()
}
