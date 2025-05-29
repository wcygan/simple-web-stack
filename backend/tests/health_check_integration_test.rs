use axum::body::Body;
use axum::http::{Request, StatusCode};
use backend::create_test_router; // Use the crate name 'backend'
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_health_check() {
    let app = create_test_router();

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("\"status\":\"healthy\""));
    assert!(body_str.contains("\"service\":\"simple-web-stack-backend\""));
}
