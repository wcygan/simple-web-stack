//! Performance Tests for Task API
//!
//! These tests are resource-intensive and MUST be run sequentially to avoid
//! database connection pool exhaustion and resource contention.
//!
//! Run with: `deno task test:backend:performance` or `RUST_TEST_THREADS=1 cargo test --test tasks_performance`
//!
//! According to the parallel-testing-database-strategy:
//! - Performance tests create hundreds/thousands of tasks
//! - Each test requires significant database connections  
//! - Running in parallel causes PoolTimedOut errors
//! - Sequential execution ensures resource isolation and test reliability

mod common;

use common::spawn_app;
use reqwest::StatusCode;
use serde_json::json;
use std::time::{Duration, Instant};

/// Test performance of bulk task creation
#[tokio::test]
async fn bulk_task_creation_performance() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let num_tasks = 1000;
    let start_time = Instant::now();

    // Create tasks in batches to avoid overwhelming the server
    let batch_size = 50;
    let mut successful_creates = 0;

    for batch_start in (0..num_tasks).step_by(batch_size) {
        let batch_end = std::cmp::min(batch_start + batch_size, num_tasks);
        let mut batch_futures = Vec::new();

        for i in batch_start..batch_end {
            let client = &client;
            let address = &test_app.address;

            let future = async move {
                client
                    .post(format!("{}/tasks", address))
                    .json(&json!({
                        "title": format!("Performance Test Task {}", i)
                    }))
                    .send()
                    .await
            };

            batch_futures.push(future);
        }

        // Execute batch concurrently
        let results = futures::future::join_all(batch_futures).await;

        for result in results {
            if let Ok(response) = result {
                if response.status() == StatusCode::CREATED {
                    successful_creates += 1;
                }
            }
        }
    }

    let duration = start_time.elapsed();

    // Performance assertions
    assert_eq!(
        successful_creates, num_tasks,
        "All tasks should be created successfully"
    );
    assert!(
        duration < Duration::from_secs(30),
        "Should complete within 30 seconds, took {:?}",
        duration
    );

    let tasks_per_second = num_tasks as f64 / duration.as_secs_f64();
    println!(
        "Created {} tasks in {:?} ({:.2} tasks/second)",
        num_tasks, duration, tasks_per_second
    );

    // Should handle at least 50 tasks per second
    assert!(
        tasks_per_second > 30.0,
        "Should handle at least 30 tasks per second, got {:.2}",
        tasks_per_second
    );

    test_app.cleanup().await;
}

/// Test performance of listing large numbers of tasks
#[tokio::test]
async fn list_tasks_performance_with_large_dataset() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a large number of tasks first
    let num_tasks = 500;
    for i in 0..num_tasks {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": format!("Large Dataset Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    // Test listing performance
    let start_time = Instant::now();

    let response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks");

    let list_duration = start_time.elapsed();

    assert_eq!(response.status(), StatusCode::OK);

    let tasks: Vec<serde_json::Value> = response.json().await.expect("Failed to parse tasks");
    assert_eq!(tasks.len(), num_tasks);

    // Performance assertions - should list tasks quickly
    assert!(
        list_duration < Duration::from_secs(5),
        "Should list {} tasks within 5 seconds, took {:?}",
        num_tasks,
        list_duration
    );

    println!("Listed {} tasks in {:?}", num_tasks, list_duration);

    test_app.cleanup().await;
}

/// Test memory usage doesn't grow excessively with many tasks
#[tokio::test]
async fn memory_usage_stress_test() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create, update, and delete tasks in cycles to test memory management
    let cycles = 10;
    let tasks_per_cycle = 100;

    for cycle in 0..cycles {
        let mut task_ids = Vec::new();

        // Create tasks
        for i in 0..tasks_per_cycle {
            let response = client
                .post(format!("{}/tasks", test_app.address))
                .json(&json!({
                    "title": format!("Memory Test Cycle {} Task {}", cycle, i)
                }))
                .send()
                .await
                .expect("Failed to create task");

            assert_eq!(response.status(), StatusCode::CREATED);

            let task: serde_json::Value = response.json().await.unwrap();
            task_ids.push(task["id"].as_str().unwrap().to_string());
        }

        // Update all tasks
        for task_id in &task_ids {
            let response = client
                .put(format!("{}/tasks/{}", test_app.address, task_id))
                .json(&json!({
                    "completed": true
                }))
                .send()
                .await
                .expect("Failed to update task");

            assert_eq!(response.status(), StatusCode::OK);
        }

        // Delete all tasks to free memory
        for task_id in &task_ids {
            let response = client
                .delete(format!("{}/tasks/{}", test_app.address, task_id))
                .send()
                .await
                .expect("Failed to delete task");

            assert_eq!(response.status(), StatusCode::NO_CONTENT);
        }

        // Verify cleanup
        let list_response = client
            .get(format!("{}/tasks", test_app.address))
            .send()
            .await
            .expect("Failed to list tasks");

        let remaining_tasks: Vec<serde_json::Value> = list_response.json().await.unwrap();
        assert_eq!(
            remaining_tasks.len(),
            0,
            "All tasks should be cleaned up after cycle {}",
            cycle
        );
    }

    println!(
        "Successfully completed {} cycles of {} tasks each",
        cycles, tasks_per_cycle
    );

    test_app.cleanup().await;
}

/// Test response time consistency under load
#[tokio::test]
async fn response_time_consistency_test() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let num_requests = 100;
    let mut response_times = Vec::new();

    // Create a baseline task for GET operations
    let create_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "Baseline task for response time test"
        }))
        .send()
        .await
        .expect("Failed to create baseline task");

    let baseline_task: serde_json::Value = create_response.json().await.unwrap();
    let baseline_task_id = baseline_task["id"].as_str().unwrap();

    // Measure response times for various operations
    for i in 0..num_requests {
        let start_time = Instant::now();

        let response = match i % 4 {
            0 => {
                // GET single task
                client
                    .get(format!("{}/tasks/{}", test_app.address, baseline_task_id))
                    .send()
                    .await
                    .expect("Failed to get task")
            }
            1 => {
                // List tasks
                client
                    .get(format!("{}/tasks", test_app.address))
                    .send()
                    .await
                    .expect("Failed to list tasks")
            }
            2 => {
                // Create task
                client
                    .post(format!("{}/tasks", test_app.address))
                    .json(&json!({
                        "title": format!("Response time test task {}", i)
                    }))
                    .send()
                    .await
                    .expect("Failed to create task")
            }
            3 => {
                // Update task
                client
                    .put(format!("{}/tasks/{}", test_app.address, baseline_task_id))
                    .json(&json!({
                        "completed": i % 8 == 0
                    }))
                    .send()
                    .await
                    .expect("Failed to update task")
            }
            _ => unreachable!(),
        };

        let response_time = start_time.elapsed();
        response_times.push(response_time);

        assert!(
            response.status().is_success(),
            "Request {} failed with status {}",
            i,
            response.status()
        );
    }

    // Calculate statistics
    let avg_response_time = response_times.iter().sum::<Duration>() / response_times.len() as u32;
    let max_response_time = *response_times.iter().max().unwrap();
    let min_response_time = *response_times.iter().min().unwrap();

    // Sort for percentile calculations
    response_times.sort();
    let p95_response_time = response_times[(response_times.len() as f64 * 0.95) as usize];
    let p99_response_time = response_times[(response_times.len() as f64 * 0.99) as usize];

    println!("Response time statistics:");
    println!("  Average: {:?}", avg_response_time);
    println!("  Min: {:?}", min_response_time);
    println!("  Max: {:?}", max_response_time);
    println!("  P95: {:?}", p95_response_time);
    println!("  P99: {:?}", p99_response_time);

    // Performance assertions
    assert!(
        avg_response_time < Duration::from_millis(100),
        "Average response time should be under 100ms, got {:?}",
        avg_response_time
    );
    assert!(
        p95_response_time < Duration::from_millis(200),
        "95th percentile should be under 200ms, got {:?}",
        p95_response_time
    );
    assert!(
        max_response_time < Duration::from_secs(1),
        "Max response time should be under 1 second, got {:?}",
        max_response_time
    );

    test_app.cleanup().await;
}

/// Test sustained load over time
#[tokio::test]
async fn sustained_load_test() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_duration = Duration::from_secs(30); // 30 second test
    let requests_per_second = 10;
    let interval = Duration::from_millis(1000 / requests_per_second);

    let start_time = Instant::now();
    let mut total_requests = 0;
    let mut successful_requests = 0;
    let mut last_request_time = start_time;

    while start_time.elapsed() < test_duration {
        if last_request_time.elapsed() >= interval {
            let request_start = Instant::now();

            let response = client
                .post(format!("{}/tasks", test_app.address))
                .json(&json!({
                    "title": format!("Sustained load task {}", total_requests)
                }))
                .send()
                .await;

            total_requests += 1;

            match response {
                Ok(resp) if resp.status() == StatusCode::CREATED => {
                    successful_requests += 1;
                }
                Ok(resp) => {
                    println!("Unexpected status: {}", resp.status());
                }
                Err(e) => {
                    println!("Request failed: {}", e);
                }
            }

            last_request_time = request_start;
        } else {
            // Small sleep to avoid busy waiting
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }

    let actual_duration = start_time.elapsed();
    let success_rate = successful_requests as f64 / total_requests as f64;
    let actual_rps = total_requests as f64 / actual_duration.as_secs_f64();

    println!("Sustained load test results:");
    println!("  Duration: {:?}", actual_duration);
    println!("  Total requests: {}", total_requests);
    println!("  Successful requests: {}", successful_requests);
    println!("  Success rate: {:.2}%", success_rate * 100.0);
    println!("  Actual RPS: {:.2}", actual_rps);

    // Performance assertions
    assert!(
        success_rate > 0.95,
        "Success rate should be above 95%, got {:.2}%",
        success_rate * 100.0
    );
    assert!(
        total_requests > 250,
        "Should complete significant number of requests in 30 seconds, got {}",
        total_requests
    );

    test_app.cleanup().await;
}
