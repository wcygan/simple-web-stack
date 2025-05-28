use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};
use tracing::{info, instrument};

#[derive(Debug, serde::Serialize)]
pub struct HealthResponse {
    status: String,
    timestamp: String,
    service: String,
}

/// Health check endpoint
///
/// Returns the current health status of the service.
#[instrument]
pub async fn health_check_handler() -> Result<Json<Value>, StatusCode> {
    info!("Health check requested");

    let health_response_data = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        service: "simple-web-stack-backend".to_string(),
    };

    let health_response_json: Value = json!(health_response_data);
    Ok(Json(health_response_json))
}
