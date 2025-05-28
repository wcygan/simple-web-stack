use backend::config::AppConfig;
use backend::create_app;
use std::net::SocketAddr;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    let config = AppConfig::default();

    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.logging.level))
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .json(),
        )
        .init();

    let app = create_app();

    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>().unwrap(),
        config.server.port,
    ));

    info!("🚀 Server starting on http://{}", addr);

    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Successfully bound to address {}", addr);
            if let Err(e) = axum::serve(listener, app).await {
                error!("Server error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to bind to address {}: {}", addr, e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use backend::create_app;
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint_integration_in_main() {
        let app = create_app();

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
        let app = create_app();
        let request = Request::builder()
            .uri("/invalid-path-for-testing")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
