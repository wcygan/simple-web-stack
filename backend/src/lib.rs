use axum::{
    routing::get,
    Router,
};
use sqlx::MySqlPool;
use tower_http::trace::TraceLayer;

use crate::handlers::health::health_check_handler;
use crate::handlers::todos::{create_todo, delete_todo, get_todo, list_todos, update_todo};

pub mod config;
pub mod handlers;
pub mod models;

/// Create the application router for use in main and tests
pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/health", get(health_check_handler))
        .route("/api/todos", get(list_todos).post(create_todo))
        .route(
            "/api/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}

pub fn create_test_router() -> Router {
    // Create a router without real DB pool for tests that don't need it
    // For tests requiring DB, use a test-specific pool
    Router::new()
        .route("/health", get(health_check_handler))
        // Add other non-DB routes here if needed for testing
        .layer(TraceLayer::new_for_http())
}
