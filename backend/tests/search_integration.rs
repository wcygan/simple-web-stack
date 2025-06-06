use backend::models::{CreateTaskPayload, Task, TaskQueryParams};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn test_search_by_title() {
    let app = common::setup_test_app().await;

    // Create test tasks
    let task1_payload = CreateTaskPayload {
        title: "Learn Rust programming".to_string(),
    };
    let task2_payload = CreateTaskPayload {
        title: "Build web application".to_string(),
    };
    let task3_payload = CreateTaskPayload {
        title: "Deploy to production".to_string(),
    };

    // Create tasks
    let response1 = app
        .post("/tasks")
        .json(&task1_payload)
        .send()
        .await;
    assert_eq!(response1.status(), 201);

    let response2 = app
        .post("/tasks")
        .json(&task2_payload)
        .send()
        .await;
    assert_eq!(response2.status(), 201);

    let response3 = app
        .post("/tasks")
        .json(&task3_payload)
        .send()
        .await;
    assert_eq!(response3.status(), 201);

    // Test search by title
    let search_response = app
        .get("/tasks?q=Rust")
        .send()
        .await;
    assert_eq!(search_response.status(), 200);

    let search_result: serde_json::Value = search_response.json().await;
    let tasks = search_result["data"].as_array().unwrap();
    assert_eq!(tasks.len(), 1);
    assert!(tasks[0]["title"].as_str().unwrap().contains("Rust"));

    // Test search with no results
    let no_results_response = app
        .get("/tasks?q=nonexistent")
        .send()
        .await;
    assert_eq!(no_results_response.status(), 200);

    let no_results: serde_json::Value = no_results_response.json().await;
    let no_tasks = no_results["data"].as_array().unwrap();
    assert_eq!(no_tasks.len(), 0);
}

#[tokio::test]
async fn test_filter_by_status() {
    let app = common::setup_test_app().await;

    // Create test tasks
    let task1_payload = CreateTaskPayload {
        title: "Completed task".to_string(),
    };
    let task2_payload = CreateTaskPayload {
        title: "Pending task".to_string(),
    };

    // Create tasks
    let response1 = app
        .post("/tasks")
        .json(&task1_payload)
        .send()
        .await;
    assert_eq!(response1.status(), 201);
    let task1: Task = response1.json().await;

    let response2 = app
        .post("/tasks")
        .json(&task2_payload)
        .send()
        .await;
    assert_eq!(response2.status(), 201);

    // Mark first task as completed
    let update_payload = json!({
        "completed": true
    });
    let update_response = app
        .put(&format!("/tasks/{}", task1.id))
        .json(&update_payload)
        .send()
        .await;
    assert_eq!(update_response.status(), 200);

    // Test filter by completed status
    let completed_response = app
        .get("/tasks?status=completed")
        .send()
        .await;
    assert_eq!(completed_response.status(), 200);

    let completed_result: serde_json::Value = completed_response.json().await;
    let completed_tasks = completed_result["data"].as_array().unwrap();
    assert_eq!(completed_tasks.len(), 1);
    assert_eq!(completed_tasks[0]["completed"].as_bool().unwrap(), true);

    // Test filter by pending status
    let pending_response = app
        .get("/tasks?status=pending")
        .send()
        .await;
    assert_eq!(pending_response.status(), 200);

    let pending_result: serde_json::Value = pending_response.json().await;
    let pending_tasks = pending_result["data"].as_array().unwrap();
    assert_eq!(pending_tasks.len(), 1);
    assert_eq!(pending_tasks[0]["completed"].as_bool().unwrap(), false);

    // Test filter by all status
    let all_response = app
        .get("/tasks?status=all")
        .send()
        .await;
    assert_eq!(all_response.status(), 200);

    let all_result: serde_json::Value = all_response.json().await;
    let all_tasks = all_result["data"].as_array().unwrap();
    assert_eq!(all_tasks.len(), 2);
}

#[tokio::test]
async fn test_combined_search_and_filter() {
    let app = common::setup_test_app().await;

    // Create test tasks with similar titles
    let task1_payload = CreateTaskPayload {
        title: "Learn Rust basics".to_string(),
    };
    let task2_payload = CreateTaskPayload {
        title: "Learn Rust advanced".to_string(),
    };
    let task3_payload = CreateTaskPayload {
        title: "Learn Python".to_string(),
    };

    // Create tasks
    let response1 = app
        .post("/tasks")
        .json(&task1_payload)
        .send()
        .await;
    assert_eq!(response1.status(), 201);
    let task1: Task = response1.json().await;

    let response2 = app
        .post("/tasks")
        .json(&task2_payload)
        .send()
        .await;
    assert_eq!(response2.status(), 201);

    let response3 = app
        .post("/tasks")
        .json(&task3_payload)
        .send()
        .await;
    assert_eq!(response3.status(), 201);

    // Mark first Rust task as completed
    let update_payload = json!({
        "completed": true
    });
    let update_response = app
        .put(&format!("/tasks/{}", task1.id))
        .json(&update_payload)
        .send()
        .await;
    assert_eq!(update_response.status(), 200);

    // Test combined search for "Rust" with status "pending"
    let combined_response = app
        .get("/tasks?q=Rust&status=pending")
        .send()
        .await;
    assert_eq!(combined_response.status(), 200);

    let combined_result: serde_json::Value = combined_response.json().await;
    let filtered_tasks = combined_result["data"].as_array().unwrap();
    assert_eq!(filtered_tasks.len(), 1);
    assert!(filtered_tasks[0]["title"].as_str().unwrap().contains("Rust"));
    assert!(filtered_tasks[0]["title"].as_str().unwrap().contains("advanced"));
    assert_eq!(filtered_tasks[0]["completed"].as_bool().unwrap(), false);

    // Test combined search for "Rust" with status "completed"
    let completed_rust_response = app
        .get("/tasks?q=Rust&status=completed")
        .send()
        .await;
    assert_eq!(completed_rust_response.status(), 200);

    let completed_rust_result: serde_json::Value = completed_rust_response.json().await;
    let completed_rust_tasks = completed_rust_result["data"].as_array().unwrap();
    assert_eq!(completed_rust_tasks.len(), 1);
    assert!(completed_rust_tasks[0]["title"].as_str().unwrap().contains("basics"));
    assert_eq!(completed_rust_tasks[0]["completed"].as_bool().unwrap(), true);
}

#[tokio::test]
async fn test_search_with_pagination() {
    let app = common::setup_test_app().await;

    // Create multiple tasks for pagination testing
    for i in 1..=25 {
        let task_payload = CreateTaskPayload {
            title: format!("Search task {}", i),
        };
        let response = app
            .post("/tasks")
            .json(&task_payload)
            .send()
            .await;
        assert_eq!(response.status(), 201);
    }

    // Test paginated search
    let paginated_response = app
        .get("/tasks?q=Search&page=1&page_size=10")
        .send()
        .await;
    assert_eq!(paginated_response.status(), 200);

    let paginated_result: serde_json::Value = paginated_response.json().await;
    let tasks = paginated_result["data"].as_array().unwrap();
    assert_eq!(tasks.len(), 10);
    
    let pagination = &paginated_result["pagination"];
    assert_eq!(pagination["page"].as_u64().unwrap(), 1);
    assert_eq!(pagination["page_size"].as_u64().unwrap(), 10);
    assert_eq!(pagination["total_items"].as_u64().unwrap(), 25);
    assert_eq!(pagination["total_pages"].as_u64().unwrap(), 3);
    assert_eq!(pagination["has_next"].as_bool().unwrap(), true);
    assert_eq!(pagination["has_previous"].as_bool().unwrap(), false);

    // Test second page
    let page2_response = app
        .get("/tasks?q=Search&page=2&page_size=10")
        .send()
        .await;
    assert_eq!(page2_response.status(), 200);

    let page2_result: serde_json::Value = page2_response.json().await;
    let page2_tasks = page2_result["data"].as_array().unwrap();
    assert_eq!(page2_tasks.len(), 10);
    
    let page2_pagination = &page2_result["pagination"];
    assert_eq!(page2_pagination["page"].as_u64().unwrap(), 2);
    assert_eq!(page2_pagination["has_next"].as_bool().unwrap(), true);
    assert_eq!(page2_pagination["has_previous"].as_bool().unwrap(), true);
}