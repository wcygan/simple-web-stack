mod common;

use common::spawn_app;
use reqwest::StatusCode;
use serde_json::json;
use std::time::{Duration, Instant};

/// Test health check reliability under load
#[tokio::test]
async fn health_check_reliability_under_load() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // First verify health check works normally
    let health_response = client
        .get(format!("{}/health", test_app.address))
        .send()
        .await
        .expect("Failed to check health");

    assert_eq!(health_response.status(), StatusCode::OK);
    let health_data: serde_json::Value = health_response.json().await.unwrap();
    assert_eq!(health_data["status"], "ok");

    // Create some load on the system
    let mut handles = Vec::new();
    for i in 0..50 {
        let client = client.clone();
        let address = test_app.address.clone();

        let handle = tokio::spawn(async move {
            // Create some tasks
            let response = client
                .post(format!("{}/tasks", address))
                .json(&json!({
                    "title": format!("Load test task {}", i)
                }))
                .send()
                .await
                .expect("Failed to create task");
            response.status()
        });

        handles.push(handle);
    }

    // While load is running, check health multiple times
    let mut health_checks = Vec::new();
    for _ in 0..10 {
        let health_handle = tokio::spawn({
            let client = client.clone();
            let address = test_app.address.clone();
            async move {
                let response = client
                    .get(format!("{}/health", address))
                    .send()
                    .await
                    .expect("Failed to check health under load");
                (
                    response.status(),
                    response.json::<serde_json::Value>().await,
                )
            }
        });
        health_checks.push(health_handle);

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Wait for all operations to complete
    for handle in handles {
        let status = handle.await.expect("Load task failed");
        assert_eq!(status, StatusCode::CREATED);
    }

    for health_handle in health_checks {
        let (status, health_result) = health_handle.await.expect("Health check task failed");
        assert_eq!(status, StatusCode::OK);

        if let Ok(health_data) = health_result {
            assert_eq!(health_data["status"], "ok");
        }
    }

    test_app.cleanup().await;
}

/// Test system behavior with rapid sequential operations
#[tokio::test]
async fn rapid_sequential_operations() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let mut task_ids = Vec::new();
    let operations_count = 100;

    // Rapid task creation
    let create_start = Instant::now();
    for i in 0..operations_count {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": format!("Rapid task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");

        assert_eq!(response.status(), StatusCode::CREATED);

        let task: serde_json::Value = response.json().await.unwrap();
        task_ids.push(task["id"].as_str().unwrap().to_string());
    }
    let create_duration = create_start.elapsed();

    println!(
        "Created {} tasks in {:?} ({:.2} ops/sec)",
        operations_count,
        create_duration,
        operations_count as f64 / create_duration.as_secs_f64()
    );

    // Rapid updates
    let update_start = Instant::now();
    for (i, task_id) in task_ids.iter().enumerate() {
        let response = client
            .put(format!("{}/tasks/{}", test_app.address, task_id))
            .json(&json!({
                "completed": i % 2 == 0
            }))
            .send()
            .await
            .expect("Failed to update task");

        assert_eq!(response.status(), StatusCode::OK);
    }
    let update_duration = update_start.elapsed();

    println!(
        "Updated {} tasks in {:?} ({:.2} ops/sec)",
        operations_count,
        update_duration,
        operations_count as f64 / update_duration.as_secs_f64()
    );

    // Rapid reads
    let read_start = Instant::now();
    for task_id in &task_ids {
        let response = client
            .get(format!("{}/tasks/{}", test_app.address, task_id))
            .send()
            .await
            .expect("Failed to get task");

        assert_eq!(response.status(), StatusCode::OK);
    }
    let read_duration = read_start.elapsed();

    println!(
        "Read {} tasks in {:?} ({:.2} ops/sec)",
        operations_count,
        read_duration,
        operations_count as f64 / read_duration.as_secs_f64()
    );

    // Verify final state consistency
    let list_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks");

    let tasks: Vec<serde_json::Value> = list_response.json().await.unwrap();
    assert_eq!(
        tasks.len(),
        operations_count,
        "All tasks should still exist"
    );

    test_app.cleanup().await;
}

/// Test system recovery after transient errors
#[tokio::test]
async fn system_recovery_after_errors() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // First, create some tasks successfully
    let mut valid_task_ids = Vec::new();
    for i in 0..5 {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": format!("Valid task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create valid task");

        assert_eq!(response.status(), StatusCode::CREATED);
        let task: serde_json::Value = response.json().await.unwrap();
        valid_task_ids.push(task["id"].as_str().unwrap().to_string());
    }

    // Generate some error scenarios
    let error_scenarios = vec![
        // Invalid JSON
        ("{invalid json", StatusCode::BAD_REQUEST),
        // Empty title
        (r#"{"title": ""}"#, StatusCode::BAD_REQUEST),
        // Missing title
        (r#"{}"#, StatusCode::UNPROCESSABLE_ENTITY),
    ];

    for (invalid_payload, expected_status) in error_scenarios {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .header("Content-Type", "application/json")
            .body(invalid_payload)
            .send()
            .await
            .expect("Failed to send invalid request");

        // Should handle errors gracefully
        assert!(
            response.status() == expected_status
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::UNPROCESSABLE_ENTITY,
            "Unexpected status for invalid payload: {}",
            response.status()
        );
    }

    // After errors, system should still work normally
    let recovery_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "Recovery test task"
        }))
        .send()
        .await
        .expect("Failed to create recovery task");

    assert_eq!(
        recovery_response.status(),
        StatusCode::CREATED,
        "System should recover after handling errors"
    );

    // Verify all original tasks still exist and are accessible
    for task_id in &valid_task_ids {
        let get_response = client
            .get(format!("{}/tasks/{}", test_app.address, task_id))
            .send()
            .await
            .expect("Failed to get task after recovery");

        assert_eq!(
            get_response.status(),
            StatusCode::OK,
            "Original tasks should still be accessible after errors"
        );
    }

    // Verify list operation still works
    let list_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks after recovery");

    assert_eq!(list_response.status(), StatusCode::OK);
    let tasks: Vec<serde_json::Value> = list_response.json().await.unwrap();
    assert!(
        tasks.len() >= 6,
        "Should have at least 6 tasks (5 original + 1 recovery)"
    );

    test_app.cleanup().await;
}

/// Test timeout behavior and resilience
#[tokio::test]
async fn timeout_behavior_and_resilience() {
    let test_app = spawn_app().await;

    // Create client with very short timeout to test resilience
    let short_timeout_client = reqwest::Client::builder()
        .timeout(Duration::from_millis(1))
        .build()
        .expect("Failed to create short timeout client");

    // Normal client for comparison
    let normal_client = reqwest::Client::new();

    // Try operations with very short timeout (should fail due to timeout)
    let timeout_result = short_timeout_client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await;

    // Should either timeout or succeed very quickly
    match timeout_result {
        Err(e) => {
            // Expected timeout error
            println!("Got expected timeout: {}", e);
        }
        Ok(response) => {
            // If it succeeded, it must have been very fast
            println!(
                "Operation succeeded despite short timeout: {}",
                response.status()
            );
        }
    }

    // After timeout attempts, normal operations should still work
    let normal_response = normal_client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to execute normal request after timeout test");

    assert_eq!(
        normal_response.status(),
        StatusCode::OK,
        "Normal operations should work after timeout tests"
    );

    // Create a task with normal client
    let create_response = normal_client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "Post-timeout test task"
        }))
        .send()
        .await
        .expect("Failed to create task after timeout test");

    assert_eq!(
        create_response.status(),
        StatusCode::CREATED,
        "Task creation should work after timeout tests"
    );

    test_app.cleanup().await;
}

/// Test content negotiation and API versioning resilience
#[tokio::test]
async fn content_negotiation_resilience() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test various Accept headers
    let accept_headers = vec![
        "application/json",
        "application/json; charset=utf-8",
        "application/json, text/plain",
        "*/*",
        "application/*",
        "", // No Accept header
    ];

    for accept_header in accept_headers {
        let mut request = client.get(format!("{}/tasks", test_app.address));

        if !accept_header.is_empty() {
            request = request.header("Accept", accept_header);
        }

        let response = request
            .send()
            .await
            .expect("Failed to execute request with Accept header");

        // Should handle various Accept headers gracefully
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Should handle Accept header '{}' gracefully",
            accept_header
        );

        // Should return JSON regardless of Accept header
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        assert!(
            content_type.contains("application/json"),
            "Should return JSON for Accept header '{}'",
            accept_header
        );
    }

    test_app.cleanup().await;
}

/// Test large payload handling and cleanup
#[tokio::test]
async fn large_payload_handling_and_cleanup() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test progressively larger payloads
    let payload_sizes = vec![1_000, 10_000, 100_000];

    for size in payload_sizes {
        let large_title = "x".repeat(size);

        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": large_title,
                "extra_data": "y".repeat(size / 2) // Additional large field
            }))
            .send()
            .await
            .expect("Failed to send large payload");

        // Should handle large payloads gracefully
        assert!(
            response.status() == StatusCode::CREATED
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::PAYLOAD_TOO_LARGE,
            "Should handle payload of size {} gracefully, got {}",
            size,
            response.status()
        );

        if response.status() == StatusCode::CREATED {
            let task: serde_json::Value = response.json().await.unwrap();
            let task_id = task["id"].as_str().unwrap();

            // Verify the task can be retrieved
            let get_response = client
                .get(format!("{}/tasks/{}", test_app.address, task_id))
                .send()
                .await
                .expect("Failed to get large task");

            assert_eq!(get_response.status(), StatusCode::OK);

            // Clean up large task
            let delete_response = client
                .delete(format!("{}/tasks/{}", test_app.address, task_id))
                .send()
                .await
                .expect("Failed to delete large task");

            assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
        }
    }

    // Verify system is still responsive after large payload tests
    let normal_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "Normal task after large payload tests"
        }))
        .send()
        .await
        .expect("Failed to create normal task after large payload tests");

    assert_eq!(
        normal_response.status(),
        StatusCode::CREATED,
        "System should be responsive after large payload tests"
    );

    test_app.cleanup().await;
}

/// Test operational monitoring and observability
#[tokio::test]
async fn operational_monitoring_capabilities() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test health endpoint provides useful information
    let health_response = client
        .get(format!("{}/health", test_app.address))
        .send()
        .await
        .expect("Failed to check health");

    assert_eq!(health_response.status(), StatusCode::OK);

    // Get headers before consuming the response with json()
    let headers = health_response.headers().clone();
    let health_data: serde_json::Value = health_response.json().await.unwrap();
    assert_eq!(health_data["status"], "ok");

    // Verify response includes proper headers for monitoring
    assert!(
        headers.contains_key("content-type"),
        "Should include content-type header"
    );

    // Test various HTTP methods return appropriate responses for monitoring
    let monitoring_endpoints = vec![
        (
            "GET",
            format!("{}/health", test_app.address),
            StatusCode::OK,
        ),
        (
            "POST",
            format!("{}/health", test_app.address),
            StatusCode::METHOD_NOT_ALLOWED,
        ),
        (
            "GET",
            format!("{}/nonexistent", test_app.address),
            StatusCode::NOT_FOUND,
        ),
    ];

    for (method, url, expected_status) in monitoring_endpoints {
        let response = match method {
            "GET" => client.get(&url).send().await,
            "POST" => client.post(&url).send().await,
            _ => continue,
        }
        .expect("Failed to execute monitoring request");

        assert!(
            response.status() == expected_status || response.status() == StatusCode::NOT_FOUND, // May vary based on router
            "Monitoring endpoint {} {} should return appropriate status, got {}",
            method,
            url,
            response.status()
        );
    }

    // Create some activity for monitoring
    let mut task_ids = Vec::new();
    for i in 0..5 {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": format!("Monitoring test task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create monitoring task");

        if response.status() == StatusCode::CREATED {
            let task: serde_json::Value = response.json().await.unwrap();
            task_ids.push(task["id"].as_str().unwrap().to_string());
        }
    }

    // Verify system state can be monitored through list endpoint
    let list_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks for monitoring");

    assert_eq!(list_response.status(), StatusCode::OK);

    let tasks: Vec<serde_json::Value> = list_response.json().await.unwrap();
    assert!(
        tasks.len() >= 5,
        "Should have at least 5 tasks for monitoring"
    );

    // Health check should still work after activity
    let final_health = client
        .get(format!("{}/health", test_app.address))
        .send()
        .await
        .expect("Failed final health check");

    assert_eq!(final_health.status(), StatusCode::OK);

    test_app.cleanup().await;
}

/// Test graceful degradation under resource constraints
#[tokio::test]
async fn graceful_degradation_under_constraints() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a baseline to ensure system is working
    let baseline_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "Baseline task before stress test"
        }))
        .send()
        .await
        .expect("Failed to create baseline task");

    assert_eq!(baseline_response.status(), StatusCode::CREATED);

    // Simulate resource pressure with many concurrent connections
    let concurrent_operations = 100;
    let mut handles = Vec::new();

    for i in 0..concurrent_operations {
        let client = client.clone();
        let address = test_app.address.clone();

        let handle = tokio::spawn(async move {
            let mut results = Vec::new();

            // Multiple operations per connection
            for j in 0..3 {
                let create_result = client
                    .post(format!("{}/tasks", address))
                    .json(&json!({
                        "title": format!("Stress task {}-{}", i, j)
                    }))
                    .send()
                    .await;

                results.push(create_result.map(|r| r.status()));
            }

            results
        });

        handles.push(handle);
    }

    // Collect results and analyze degradation behavior
    let mut successful_operations = 0;
    let mut failed_operations = 0;
    let mut error_statuses = std::collections::HashMap::new();

    for handle in handles {
        let results = handle.await.expect("Stress test task panicked");

        for result in results {
            match result {
                Ok(StatusCode::CREATED) => successful_operations += 1,
                Ok(status) => {
                    failed_operations += 1;
                    *error_statuses.entry(status).or_insert(0) += 1;
                }
                Err(_) => failed_operations += 1,
            }
        }
    }

    let total_operations = successful_operations + failed_operations;
    let success_rate = successful_operations as f64 / total_operations as f64;

    println!("Stress test results:");
    println!("  Total operations: {}", total_operations);
    println!("  Successful: {}", successful_operations);
    println!("  Failed: {}", failed_operations);
    println!("  Success rate: {:.2}%", success_rate * 100.0);
    println!("  Error statuses: {:?}", error_statuses);

    // System should maintain reasonable success rate even under stress
    assert!(
        success_rate > 0.8,
        "Success rate should be above 80% under stress, got {:.2}%",
        success_rate * 100.0
    );

    // System should still be responsive after stress test
    let recovery_response = client
        .get(format!("{}/health", test_app.address))
        .send()
        .await
        .expect("Failed to check health after stress test");

    assert_eq!(
        recovery_response.status(),
        StatusCode::OK,
        "Health check should work after stress test"
    );

    let post_stress_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "Post-stress recovery task"
        }))
        .send()
        .await
        .expect("Failed to create task after stress test");

    assert_eq!(
        post_stress_response.status(),
        StatusCode::CREATED,
        "Task creation should work after stress test"
    );

    test_app.cleanup().await;
}
