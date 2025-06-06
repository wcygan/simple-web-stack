mod common;

use common::spawn_app;
use reqwest::StatusCode;
use serde_json::{json, Value};

#[tokio::test]
async fn list_tasks_returns_paginated_response_with_default_params() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create some tasks
    for i in 1..=5 {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");
    }

    // List tasks with default pagination
    let response = client
        .get(format!("{}/tasks", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    
    // Debug: print the actual response
    println!("Response: {:#}", paginated_response);
    
    // Check structure
    assert!(paginated_response["data"].is_array());
    assert!(paginated_response["pagination"].is_object());
    
    // Check pagination metadata
    let pagination = &paginated_response["pagination"];
    assert_eq!(pagination["page"], 1);
    assert_eq!(pagination["page_size"], 20);
    assert_eq!(pagination["total_items"], 5);
    assert_eq!(pagination["total_pages"], 1);
    assert_eq!(pagination["has_next"], false);
    assert_eq!(pagination["has_previous"], false);
    
    // Check data
    let data = paginated_response["data"].as_array().unwrap();
    assert_eq!(data.len(), 5);

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_respects_page_size_parameter() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create 10 tasks
    for i in 1..=10 {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");
    }

    // List tasks with page_size=3
    let response = client
        .get(format!("{}/tasks?page_size=3", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    
    let pagination = &paginated_response["pagination"];
    assert_eq!(pagination["page"], 1);
    assert_eq!(pagination["page_size"], 3);
    assert_eq!(pagination["total_items"], 10);
    assert_eq!(pagination["total_pages"], 4); // 10 items / 3 per page = 4 pages
    assert_eq!(pagination["has_next"], true);
    assert_eq!(pagination["has_previous"], false);
    
    let data = paginated_response["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_respects_page_parameter() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create 10 tasks
    for i in 1..=10 {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");
    }

    // List tasks on page 2 with page_size=3
    let response = client
        .get(format!("{}/tasks?page=2&page_size=3", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    
    let pagination = &paginated_response["pagination"];
    assert_eq!(pagination["page"], 2);
    assert_eq!(pagination["page_size"], 3);
    assert_eq!(pagination["total_items"], 10);
    assert_eq!(pagination["total_pages"], 4);
    assert_eq!(pagination["has_next"], true);
    assert_eq!(pagination["has_previous"], true);
    
    let data = paginated_response["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_handles_last_page_correctly() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create 7 tasks
    for i in 1..=7 {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");
    }

    // Get the last page (should have 1 item)
    let response = client
        .get(format!("{}/tasks?page=3&page_size=3", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    
    let pagination = &paginated_response["pagination"];
    assert_eq!(pagination["page"], 3);
    assert_eq!(pagination["page_size"], 3);
    assert_eq!(pagination["total_items"], 7);
    assert_eq!(pagination["total_pages"], 3); // 7 items: pages 1 and 2 have 3 each, page 3 has 1
    assert_eq!(pagination["has_next"], false);
    assert_eq!(pagination["has_previous"], true);
    
    let data = paginated_response["data"].as_array().unwrap();
    assert_eq!(data.len(), 1); // Only 1 item on the last page

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_handles_invalid_page_gracefully() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create 3 tasks
    for i in 1..=3 {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");
    }

    // Request page 0 (should default to page 1)
    let response = client
        .get(format!("{}/tasks?page=0", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    
    let pagination = &paginated_response["pagination"];
    assert_eq!(pagination["page"], 1); // Should default to 1

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_handles_invalid_page_size_gracefully() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create 3 tasks
    for i in 1..=3 {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");
    }

    // Request page_size=0 (should default to 20)
    let response = client
        .get(format!("{}/tasks?page_size=0", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    
    let pagination = &paginated_response["pagination"];
    assert_eq!(pagination["page_size"], 20); // Should default to 20

    // Request page_size=200 (should cap to 100 or default)
    let response = client
        .get(format!("{}/tasks?page_size=200", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    
    let pagination = &paginated_response["pagination"];
    assert_eq!(pagination["page_size"], 20); // Should cap to reasonable limit

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_respects_sort_by_parameter() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create tasks with different titles for sorting
    let titles = ["Zebra Task", "Alpha Task", "Beta Task"];
    for title in &titles {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": title
            }))
            .send()
            .await
            .expect("Failed to create task");
    }

    // Sort by title ascending
    let response = client
        .get(format!("{}/tasks?sort_by=title&sort_order=asc", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    let data = paginated_response["data"].as_array().unwrap();
    
    // Should be sorted alphabetically
    assert_eq!(data[0]["title"], "Alpha Task");
    assert_eq!(data[1]["title"], "Beta Task");
    assert_eq!(data[2]["title"], "Zebra Task");

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_supports_sort_order_desc() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create tasks
    for i in 1..=3 {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");
        
        // Small delay to ensure different created_at times
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // Sort by created_at descending (default order)
    let response = client
        .get(format!("{}/tasks?sort_by=created_at&sort_order=desc", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    let data = paginated_response["data"].as_array().unwrap();
    
    // Should be sorted by creation time descending (newest first)
    assert_eq!(data[0]["title"], "Task 3");
    assert_eq!(data[1]["title"], "Task 2");
    assert_eq!(data[2]["title"], "Task 1");

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_handles_empty_results() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Don't create any tasks, just test pagination on empty results
    let response = client
        .get(format!("{}/tasks?page=1&page_size=10", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let paginated_response: Value = response.json().await.expect("Failed to parse response");
    
    let pagination = &paginated_response["pagination"];
    assert_eq!(pagination["page"], 1);
    assert_eq!(pagination["page_size"], 10);
    assert_eq!(pagination["total_items"], 0);
    assert_eq!(pagination["total_pages"], 1); // At least 1 page even with no items
    assert_eq!(pagination["has_next"], false);
    assert_eq!(pagination["has_previous"], false);
    
    let data = paginated_response["data"].as_array().unwrap();
    assert_eq!(data.len(), 0);

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_performance_with_large_dataset() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create 100 tasks for performance testing
    for i in 1..=100 {
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Performance Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");
    }

    // Test pagination performance with different page sizes
    let page_sizes = [10, 20, 50];
    
    for &page_size in &page_sizes {
        let start = std::time::Instant::now();
        
        let response = client
            .get(format!("{}/tasks?page=1&page_size={}", &test_app.address, page_size))
            .send()
            .await
            .expect("Failed to execute request.");

        let duration = start.elapsed();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        let paginated_response: Value = response.json().await.expect("Failed to parse response");
        let pagination = &paginated_response["pagination"];
        
        assert_eq!(pagination["total_items"], 100);
        assert_eq!(pagination["page_size"], page_size);
        
        let data = paginated_response["data"].as_array().unwrap();
        assert_eq!(data.len(), page_size as usize);
        
        // Performance assertion: should complete within reasonable time
        assert!(duration.as_millis() < 1000, "Query took too long: {:?}", duration);
    }

    test_app.cleanup().await;
}