mod common;

use common::spawn_app;
use serde_json::json;

#[tokio::test]
async fn health_check_works() {
    // Spawn the app
    let test_app = spawn_app().await;

    // Create a client
    let client = reqwest::Client::new();

    // Send request to health endpoint
    let response = client
        .get(format!("{}/health", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Check the response
    assert!(response.status().is_success());

    let json_response: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    assert_eq!(json_response, json!({"status": "ok"}));

    // Cleanup
    test_app.cleanup().await;
}

#[tokio::test]
async fn health_check_returns_200() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);

    test_app.cleanup().await;
}
