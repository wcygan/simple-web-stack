use axum::{routing::get, Router};

pub mod config;
pub mod handlers;

/// Create the application router for use in main and tests
pub fn create_app() -> Router {
    Router::new().route("/health", get(handlers::health::health_check_handler))
}
