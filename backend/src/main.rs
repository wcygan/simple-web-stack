use backend::config::AppConfig;
use backend::create_router;
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use std::time::Duration;
use tracing::info;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load application configuration
    let config = AppConfig::from_env()?;

    // Create database connection pool
    let db_pool = MySqlPoolOptions::new()
        .max_connections(config.database.max_connections)
        .acquire_timeout(Duration::from_secs(config.database.connect_timeout))
        .connect(&config.database.url)
        .await
        .expect("Failed to create database connection pool");

    // Run database migrations
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");
    info!("Database migrations completed.");

    // Create the application router
    let app = create_router(db_pool.clone());

    // Define the server address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Server listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
     // Import items from parent module
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use backend::create_test_router;
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint_integration_in_main() {
        let app = create_test_router();

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json_response: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json_response["status"], "healthy");
        assert_eq!(json_response["service"], "simple-web-stack-backend");
        assert!(json_response["timestamp"].is_string());
    }

    #[tokio::test]
    async fn test_invalid_endpoint_in_main() {
        let app = create_test_router();
        let request = Request::builder()
            .uri("/invalid-path-for-testing")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
