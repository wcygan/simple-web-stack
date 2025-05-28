use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/health", get(health_check));

    // Define the address to bind to
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("🚀 Server starting on http://{}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Health check endpoint
async fn health_check() -> Result<Json<Value>, StatusCode> {
    let health_response: Value = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "simple-web-stack-backend"
    });

    Ok(Json(health_response))
}

/// Create the application router for testing
pub fn create_app() -> Router {
    Router::new().route("/health", get(health_check))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_health_check_handler() {
        // Test the health_check handler directly
        let result = health_check().await;

        assert!(result.is_ok());

        let Json(response) = result.unwrap();

        // Verify response structure
        assert_eq!(response["status"], "healthy");
        assert_eq!(response["service"], "simple-web-stack-backend");
        assert!(response["timestamp"].is_string());

        // Verify timestamp is a valid RFC3339 format
        let timestamp_str = response["timestamp"].as_str().unwrap();
        assert!(chrono::DateTime::parse_from_rfc3339(timestamp_str).is_ok());
    }

    #[tokio::test]
    async fn test_health_endpoint_integration() {
        let app = create_app();

        // Create a request to the health endpoint
        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        // Send the request to the app
        let response = app.oneshot(request).await.unwrap();

        // Verify status code
        assert_eq!(response.status(), StatusCode::OK);

        // Verify content type
        let content_type = response.headers().get("content-type").unwrap();
        assert_eq!(content_type, "application/json");

        // Read and parse the response body
        let body = response.into_body();
        let body_bytes = body.collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        let json_response: Value = serde_json::from_str(&body_str).unwrap();

        // Verify JSON structure
        assert_eq!(json_response["status"], "healthy");
        assert_eq!(json_response["service"], "simple-web-stack-backend");
        assert!(json_response["timestamp"].is_string());
    }

    #[tokio::test]
    async fn test_invalid_endpoint() {
        let app = create_app();

        // Create a request to a non-existent endpoint
        let request = Request::builder()
            .uri("/invalid")
            .body(Body::empty())
            .unwrap();

        // Send the request to the app
        let response = app.oneshot(request).await.unwrap();

        // Verify 404 status code
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_health_endpoint_multiple_calls() {
        // Test that multiple calls to health endpoint work correctly
        let app = create_app();

        for _ in 0..3 {
            let request = Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap();

            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn test_health_response_json_schema() {
        let app = create_app();

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        let body = response.into_body();
        let body_bytes = body.collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        let json_response: Value = serde_json::from_str(&body_str).unwrap();

        // Verify all required fields are present
        assert!(json_response.get("status").is_some());
        assert!(json_response.get("timestamp").is_some());
        assert!(json_response.get("service").is_some());

        // Verify field types
        assert!(json_response["status"].is_string());
        assert!(json_response["timestamp"].is_string());
        assert!(json_response["service"].is_string());

        // Verify no unexpected fields (should only have 3 fields)
        assert_eq!(json_response.as_object().unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_timestamp_format_validity() {
        let app = create_app();

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        let body = response.into_body();
        let body_bytes = body.collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        let json_response: Value = serde_json::from_str(&body_str).unwrap();
        let timestamp_str = json_response["timestamp"].as_str().unwrap();

        // Parse timestamp to ensure it's valid RFC3339
        let parsed_timestamp = chrono::DateTime::parse_from_rfc3339(timestamp_str);
        assert!(parsed_timestamp.is_ok());

        // Verify timestamp is recent (within last minute)
        let now = chrono::Utc::now();
        let timestamp = parsed_timestamp.unwrap().with_timezone(&chrono::Utc);
        let duration = now.signed_duration_since(timestamp);
        assert!(duration.num_seconds() < 60);
    }
}
