mod common;

use common::spawn_app;
use reqwest::StatusCode;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Barrier;

/// Test concurrent task creation to ensure no race conditions
#[tokio::test]
async fn concurrent_task_creation_succeeds() {
    let test_app = spawn_app().await;
    let client = Arc::new(reqwest::Client::new());
    let barrier = Arc::new(Barrier::new(10));

    let mut handles = Vec::new();

    // Spawn 10 concurrent task creation operations
    for i in 0..10 {
        let client = client.clone();
        let address = test_app.address.clone();
        let barrier = barrier.clone();

        let handle = tokio::spawn(async move {
            barrier.wait().await; // Ensure all tasks start simultaneously

            let response = client
                .post(format!("{}/tasks", address))
                .json(&json!({
                    "title": format!("Concurrent Task {}", i)
                }))
                .send()
                .await
                .expect("Failed to execute request");

            (
                response.status(),
                response.json::<serde_json::Value>().await,
            )
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    let mut successful_creates = 0;
    let mut task_ids = Vec::new();

    for handle in handles {
        let (status, task_result) = handle.await.expect("Task panicked");

        if status == StatusCode::CREATED {
            successful_creates += 1;
            if let Ok(task) = task_result {
                task_ids.push(task["id"].as_str().unwrap().to_string());
            }
        }
    }

    // All concurrent creates should succeed
    assert_eq!(successful_creates, 10);

    // Verify all tasks were actually created and have unique IDs
    assert_eq!(task_ids.len(), 10);
    let unique_ids: std::collections::HashSet<_> = task_ids.iter().collect();
    assert_eq!(unique_ids.len(), 10, "All task IDs should be unique");

    test_app.cleanup().await;
}

/// Test concurrent updates to the same task
#[tokio::test]
async fn concurrent_task_updates_maintain_consistency() {
    let test_app = spawn_app().await;
    let client = Arc::new(reqwest::Client::new());

    // Create a task first
    let create_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "Task for concurrent updates"
        }))
        .send()
        .await
        .expect("Failed to create task");

    let created_task: serde_json::Value = create_response.json().await.unwrap();
    let task_id = created_task["id"].as_str().unwrap().to_string();

    let barrier = Arc::new(Barrier::new(5));
    let mut handles = Vec::new();

    // Spawn 5 concurrent update operations
    for i in 0..5 {
        let client = client.clone();
        let address = test_app.address.clone();
        let task_id = task_id.clone();
        let barrier = barrier.clone();

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            let response = client
                .put(format!("{}/tasks/{}", address, task_id))
                .json(&json!({
                    "title": format!("Updated by thread {}", i),
                    "completed": i % 2 == 0
                }))
                .send()
                .await
                .expect("Failed to execute request");

            (
                response.status(),
                response.json::<serde_json::Value>().await,
            )
        });

        handles.push(handle);
    }

    // Collect results
    let mut successful_updates = 0;
    for handle in handles {
        let (status, _) = handle.await.expect("Task panicked");
        if status == StatusCode::OK {
            successful_updates += 1;
        }
    }

    // All updates should succeed (last writer wins)
    assert_eq!(successful_updates, 5);

    // Verify final state is consistent
    let final_response = client
        .get(format!("{}/tasks/{}", test_app.address, task_id))
        .send()
        .await
        .expect("Failed to get final task state");

    assert_eq!(final_response.status(), StatusCode::OK);
    let final_task: serde_json::Value = final_response.json().await.unwrap();

    // Should have a title from one of the updates
    let title = final_task["title"].as_str().unwrap();
    assert!(title.starts_with("Updated by thread"));

    test_app.cleanup().await;
}

/// Test concurrent read and write operations
#[tokio::test]
async fn concurrent_read_write_operations() {
    let test_app = spawn_app().await;
    let client = Arc::new(reqwest::Client::new());

    // Create initial tasks
    let mut initial_task_ids = Vec::new();
    for i in 0..5 {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": format!("Initial Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create initial task");

        let task: serde_json::Value = response.json().await.unwrap();
        initial_task_ids.push(task["id"].as_str().unwrap().to_string());
    }

    let barrier = Arc::new(Barrier::new(20));
    let mut handles = Vec::new();

    // Spawn mixed read and write operations
    for i in 0..20 {
        let client = client.clone();
        let address = test_app.address.clone();
        let task_ids = initial_task_ids.clone();
        let barrier = barrier.clone();

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            match i % 4 {
                0 => {
                    // List all tasks
                    let response = client
                        .get(format!("{}/tasks", address))
                        .send()
                        .await
                        .expect("Failed to list tasks");
                    assert_eq!(response.status(), StatusCode::OK);
                }
                1 => {
                    // Read specific task
                    let task_id = &task_ids[i % task_ids.len()];
                    let response = client
                        .get(format!("{}/tasks/{}", address, task_id))
                        .send()
                        .await
                        .expect("Failed to get task");
                    assert!(
                        response.status() == StatusCode::OK
                            || response.status() == StatusCode::NOT_FOUND
                    );
                }
                2 => {
                    // Update task
                    let task_id = &task_ids[i % task_ids.len()];
                    let response = client
                        .put(format!("{}/tasks/{}", address, task_id))
                        .json(&json!({
                            "completed": true
                        }))
                        .send()
                        .await
                        .expect("Failed to update task");
                    assert!(
                        response.status() == StatusCode::OK
                            || response.status() == StatusCode::NOT_FOUND
                    );
                }
                3 => {
                    // Create new task
                    let response = client
                        .post(format!("{}/tasks", address))
                        .json(&json!({
                            "title": format!("Concurrent Task {}", i)
                        }))
                        .send()
                        .await
                        .expect("Failed to create task");
                    assert_eq!(response.status(), StatusCode::CREATED);
                }
                _ => unreachable!(),
            }
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.expect("Task panicked");
    }

    // Verify system is still consistent
    let final_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to get final task list");

    assert_eq!(final_response.status(), StatusCode::OK);
    let tasks: Vec<serde_json::Value> = final_response.json().await.unwrap();

    // Should have at least the initial 5 tasks plus any created during concurrent operations
    assert!(
        tasks.len() >= 5,
        "Should have at least 5 tasks after concurrent operations"
    );

    test_app.cleanup().await;
}

/// Test concurrent delete operations
#[tokio::test]
async fn concurrent_delete_operations() {
    let test_app = spawn_app().await;
    let client = Arc::new(reqwest::Client::new());

    // Create tasks to delete
    let mut task_ids = Vec::new();
    for i in 0..10 {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": format!("Task to delete {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");

        let task: serde_json::Value = response.json().await.unwrap();
        task_ids.push(task["id"].as_str().unwrap().to_string());
    }

    let barrier = Arc::new(Barrier::new(10));
    let mut handles = Vec::new();

    // Spawn concurrent delete operations
    for task_id in task_ids {
        let client = client.clone();
        let address = test_app.address.clone();
        let barrier = barrier.clone();

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            let response = client
                .delete(format!("{}/tasks/{}", address, task_id))
                .send()
                .await
                .expect("Failed to execute delete request");

            response.status()
        });

        handles.push(handle);
    }

    // Collect results
    let mut successful_deletes = 0;
    let mut not_found_responses = 0;

    for handle in handles {
        let status = handle.await.expect("Task panicked");
        match status {
            StatusCode::NO_CONTENT => successful_deletes += 1,
            StatusCode::NOT_FOUND => not_found_responses += 1,
            _ => panic!("Unexpected status code: {}", status),
        }
    }

    // Each task should be deleted exactly once
    assert_eq!(successful_deletes, 10);
    assert_eq!(not_found_responses, 0);

    // Verify all tasks are gone
    let final_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to get final task list");

    let tasks: Vec<serde_json::Value> = final_response.json().await.unwrap();
    assert_eq!(tasks.len(), 0, "All tasks should be deleted");

    test_app.cleanup().await;
}
