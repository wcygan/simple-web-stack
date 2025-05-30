mod common;

use common::spawn_app;
use reqwest::StatusCode;
use serde_json::json;

/// Test SQL injection attempts in task titles
#[tokio::test]
async fn sql_injection_protection_in_title() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let malicious_payloads: Vec<&'static str> = vec![
        "'; DROP TABLE tasks; --",
        "' OR '1'='1",
        "'; UPDATE tasks SET completed = true; --",
        "' UNION SELECT * FROM tasks --",
        "admin'--",
        "admin';#",
        "admin'/*",
        "' OR 1=1#",
        "' OR 1=1--",
        "'; EXEC xp_cmdshell('dir'); --",
    ];

    for payload in malicious_payloads {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": payload
            }))
            .send()
            .await
            .expect("Failed to execute request");

        // Should either succeed (if properly escaped) or fail with validation error
        // But should never cause SQL injection
        assert!(
            response.status() == StatusCode::CREATED
                || response.status() == StatusCode::BAD_REQUEST,
            "Unexpected status for payload '{}': {}",
            payload,
            response.status()
        );

        if response.status() == StatusCode::CREATED {
            let task: serde_json::Value = response.json().await.unwrap();
            // The payload should be stored as-is (escaped) or sanitized
            assert!(task["title"].is_string(), "Title should be a string");
        }
    }

    // Verify the tasks table still exists and functions
    let list_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks after injection attempts");

    assert_eq!(
        list_response.status(),
        StatusCode::OK,
        "Tasks endpoint should still work after injection attempts"
    );

    test_app.cleanup().await;
}

/// Test extremely long input handling
#[tokio::test]
async fn extreme_length_input_handling() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test various extreme lengths
    let test_cases = vec![
        (1000, "Should handle reasonable long titles"),
        (10000, "Should handle very long titles"),
        (100000, "Should handle extremely long titles"),
        (1000000, "Should handle massive titles"),
    ];

    for (length, description) in test_cases {
        let long_title = "a".repeat(length);

        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": long_title
            }))
            .send()
            .await
            .expect("Failed to execute request");

        // Should either accept or reject gracefully (not crash)
        assert!(
            response.status() == StatusCode::CREATED
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::PAYLOAD_TOO_LARGE,
            "{}: Unexpected status for length {}: {}",
            description,
            length,
            response.status()
        );

        // If accepted, verify it can be retrieved
        if response.status() == StatusCode::CREATED {
            let task: serde_json::Value = response.json().await.unwrap();
            let task_id = task["id"].as_str().unwrap();

            let get_response = client
                .get(format!("{}/tasks/{}", test_app.address, task_id))
                .send()
                .await
                .expect("Failed to get created task");

            assert_eq!(get_response.status(), StatusCode::OK);

            let retrieved_task: serde_json::Value = get_response.json().await.unwrap();
            // Title should be preserved or truncated consistently
            assert!(retrieved_task["title"].is_string());
        }
    }

    test_app.cleanup().await;
}

/// Test Unicode and special character handling
#[tokio::test]
async fn unicode_and_special_character_handling() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let unicode_test_cases = vec![
        "🚀 Task with emojis 🎉",
        "Task with áccénts and ñ",
        "Задача на русском языке",
        "中文任务标题",
        "タスクのタイトル",
        "مهمة باللغة العربية",
        "נושא במשימה עברית",
        "\"Task with quotes\"",
        "Task with 'single quotes'",
        "Task with\nnewlines\r\n",
        "Task with\ttabs",
        "Task with \x00 null bytes",
        "Task with control chars \x01\x02\x03",
        "\u{2028}\u{2029}",      // Line/paragraph separators
        "\u{FEFF}Task with BOM", // Byte order mark
    ];

    for title in unicode_test_cases {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": title
            }))
            .send()
            .await
            .expect("Failed to execute request");

        // Should handle unicode gracefully
        assert!(
            response.status() == StatusCode::CREATED
                || response.status() == StatusCode::BAD_REQUEST,
            "Unexpected status for unicode title '{}': {}",
            title.chars().take(50).collect::<String>(),
            response.status()
        );

        if response.status() == StatusCode::CREATED {
            let task: serde_json::Value = response.json().await.unwrap();
            let task_id = task["id"].as_str().unwrap();

            // Verify the task can be retrieved and updated
            let get_response = client
                .get(format!("{}/tasks/{}", test_app.address, task_id))
                .send()
                .await
                .expect("Failed to get unicode task");

            assert_eq!(get_response.status(), StatusCode::OK);
        }
    }

    test_app.cleanup().await;
}

/// Test malformed JSON and content type attacks
#[tokio::test]
async fn malformed_json_handling() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let malformed_payloads = vec![
        ("", "Empty body"),
        ("{", "Incomplete JSON"),
        ("{'title': 'test'}", "Single quotes"),
        ("{\"title\": }", "Missing value"),
        ("{\"title\": \"test\", }", "Trailing comma"),
        ("null", "Null JSON"),
        ("[]", "Array instead of object"),
        ("\"string\"", "String instead of object"),
        ("123", "Number instead of object"),
        (
            "{\"title\": \"test\", \"title\": \"duplicate\"}",
            "Duplicate keys",
        ),
        ("{\n\"title\": \"test\"\n}", "Valid JSON with newlines"),
    ];

    for (payload, description) in malformed_payloads {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .header("Content-Type", "application/json")
            .body(payload)
            .send()
            .await
            .expect("Failed to execute request");

        // Should return appropriate error status for malformed JSON
        assert!(
            response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::UNPROCESSABLE_ENTITY
                || response.status() == StatusCode::CREATED, // For valid JSON
            "{}: Unexpected status for payload '{}': {}",
            description,
            payload,
            response.status()
        );
    }

    test_app.cleanup().await;
}

/// Test HTTP method security
#[tokio::test]
async fn http_method_security() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test unsupported methods on task endpoints
    let unsupported_methods = vec![
        (
            client.patch(format!("{}/tasks", test_app.address)),
            "PATCH on collection",
        ),
        (
            client.patch(format!("{}/tasks/123", test_app.address)),
            "PATCH on item",
        ),
    ];

    for (request_builder, description) in unsupported_methods {
        let response = request_builder
            .send()
            .await
            .expect("Failed to execute request");

        // Should return Method Not Allowed or Not Found
        assert!(
            response.status() == StatusCode::METHOD_NOT_ALLOWED
                || response.status() == StatusCode::NOT_FOUND,
            "{}: Unexpected status: {}",
            description,
            response.status()
        );
    }

    // Test that HEAD requests work for GET endpoints (this is correct HTTP behavior)
    let head_requests = vec![
        (
            client.head(format!("{}/tasks", test_app.address)),
            "HEAD on collection",
        ),
        (
            client.head(format!("{}/tasks/123", test_app.address)),
            "HEAD on item",
        ),
    ];

    for (request_builder, description) in head_requests {
        let response = request_builder
            .send()
            .await
            .expect("Failed to execute request");

        // HEAD should return 200 (for collection) or 400/404 (for invalid UUID)
        assert!(
            response.status() == StatusCode::OK
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::NOT_FOUND,
            "{}: Unexpected status: {}",
            description,
            response.status()
        );
    }

    test_app.cleanup().await;
}

/// Test content type validation
#[tokio::test]
async fn content_type_validation() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let invalid_content_types = vec![
        "application/xml",
        "text/plain",
        "text/html",
        "application/x-www-form-urlencoded",
        "multipart/form-data",
        "application/octet-stream",
        "", // No content type
    ];

    for content_type in invalid_content_types {
        let mut request = client
            .post(format!("{}/tasks", test_app.address))
            .body("{\"title\": \"test\"}");

        if !content_type.is_empty() {
            request = request.header("Content-Type", content_type);
        }

        let response = request.send().await.expect("Failed to execute request");

        // Should return appropriate error for unsupported content types
        // or succeed if the server is lenient
        assert!(
            response.status() == StatusCode::UNSUPPORTED_MEDIA_TYPE
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::CREATED, // If server is lenient
            "Unexpected status for content type '{}': {}",
            content_type,
            response.status()
        );
    }

    test_app.cleanup().await;
}

/// Test UUID format validation
#[tokio::test]
async fn uuid_format_validation() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let invalid_uuids = vec![
        "123",
        "not-a-uuid",
        "12345678-1234-1234-1234-12345678901",   // Too short
        "12345678-1234-1234-1234-1234567890123", // Too long
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",  // Invalid characters
        "12345678-1234-1234-1234-12345678901g",  // Invalid character at end
        "12345678_1234_1234_1234_123456789012",  // Wrong separators
        "123456781234123412341234567890ab",      // No separators
        "",                                      // Empty
        "../../../../etc/passwd",                // Path traversal attempt
        "SELECT * FROM tasks",                   // SQL injection attempt
        "../admin",                              // Directory traversal
    ];

    for invalid_uuid in invalid_uuids {
        // Test GET with invalid UUID
        let get_response = client
            .get(format!("{}/tasks/{}", test_app.address, invalid_uuid))
            .send()
            .await
            .expect("Failed to execute GET request");

        assert!(
            get_response.status() == StatusCode::BAD_REQUEST
                || get_response.status() == StatusCode::NOT_FOUND,
            "GET: Unexpected status for invalid UUID '{}': {}",
            invalid_uuid,
            get_response.status()
        );

        // Test PUT with invalid UUID
        let put_response = client
            .put(format!("{}/tasks/{}", test_app.address, invalid_uuid))
            .json(&json!({"title": "test"}))
            .send()
            .await
            .expect("Failed to execute PUT request");

        assert!(
            put_response.status() == StatusCode::BAD_REQUEST
                || put_response.status() == StatusCode::NOT_FOUND,
            "PUT: Unexpected status for invalid UUID '{}': {}",
            invalid_uuid,
            put_response.status()
        );

        // Test DELETE with invalid UUID
        let delete_response = client
            .delete(format!("{}/tasks/{}", test_app.address, invalid_uuid))
            .send()
            .await
            .expect("Failed to execute DELETE request");

        assert!(
            delete_response.status() == StatusCode::BAD_REQUEST
                || delete_response.status() == StatusCode::NOT_FOUND,
            "DELETE: Unexpected status for invalid UUID '{}': {}",
            invalid_uuid,
            delete_response.status()
        );
    }

    test_app.cleanup().await;
}

/// Test request size limits
#[tokio::test]
async fn request_size_limits() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test extremely large JSON payload
    let large_object = json!({
        "title": "Normal title",
        "extra_field": "x".repeat(1_000_000), // 1MB of extra data
        "another_field": vec!["data"; 10000], // Large array
        "nested": {
            "deep": {
                "very": {
                    "deeply": {
                        "nested": {
                            "data": "x".repeat(100000)
                        }
                    }
                }
            }
        }
    });

    let response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&large_object)
        .send()
        .await
        .expect("Failed to execute request");

    // Should handle large payloads gracefully
    assert!(
        response.status() == StatusCode::PAYLOAD_TOO_LARGE
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::CREATED, // If server accepts large payloads
        "Unexpected status for large payload: {}",
        response.status()
    );

    test_app.cleanup().await;
}

/// Test XSS prevention in API responses
#[tokio::test]
async fn xss_prevention_in_responses() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let xss_payloads = vec![
        "<script>alert('xss')</script>",
        "<img src=x onerror=alert('xss')>",
        "javascript:alert('xss')",
        "<svg onload=alert('xss')>",
        "';alert('xss');//",
        "\"><script>alert('xss')</script>",
        "<iframe src=javascript:alert('xss')></iframe>",
    ];

    for payload in xss_payloads {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": payload
            }))
            .send()
            .await
            .expect("Failed to execute request");

        if response.status() == StatusCode::CREATED {
            let task: serde_json::Value = response.json().await.unwrap();
            let task_id = task["id"].as_str().unwrap();

            // Verify the response is properly JSON-encoded (not executed as script)
            let response_text = serde_json::to_string(&task).unwrap();

            // JSON should properly quote the content, making it safe
            // The payload should be stored as a string value, not executable code
            assert!(
                response_text.contains(&format!("\"{}\"", payload.replace("\"", "\\\""))),
                "XSS payload should be properly JSON-encoded in response: {}",
                payload
            );

            // Verify the title field contains the original payload as a string
            assert_eq!(
                task["title"].as_str().unwrap(),
                payload,
                "Title should contain the original payload as a string"
            );

            // Verify retrieval also handles it properly
            let get_response = client
                .get(format!("{}/tasks/{}", test_app.address, task_id))
                .send()
                .await
                .expect("Failed to get task");

            if get_response.status() == StatusCode::OK {
                let retrieved_task: serde_json::Value = get_response.json().await.unwrap();

                // Verify the retrieved task also has the payload as a string
                assert_eq!(
                    retrieved_task["title"].as_str().unwrap(),
                    payload,
                    "Retrieved title should contain the original payload as a string"
                );

                // Verify JSON encoding is proper
                let get_response_text = serde_json::to_string(&retrieved_task).unwrap();
                assert!(
                    get_response_text.contains(&format!("\"{}\"", payload.replace("\"", "\\\""))),
                    "XSS payload should be properly JSON-encoded in GET response: {}",
                    payload
                );
            }
        }
    }

    test_app.cleanup().await;
}
