mod common;

use common::spawn_app;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn create_task_returns_201_for_valid_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Test task"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);

    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(task["title"], "Test task");
    assert_eq!(task["completed"], false);
    assert!(task["id"].is_string());

    test_app.cleanup().await;
}

#[tokio::test]
async fn create_task_returns_400_for_empty_title() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "   "
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    test_app.cleanup().await;
}

#[tokio::test]
async fn create_task_returns_422_for_missing_title() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/tasks", &test_app.address))
        .json(&json!({}))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    test_app.cleanup().await;
}

#[tokio::test]
async fn get_task_returns_200_for_existing_task() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(&format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Test task"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    let created_task: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse response");
    let task_id = created_task["id"].as_str().unwrap();

    // Get the task
    let response = client
        .get(&format!("{}/tasks/{}", &test_app.address, task_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(task["id"], task_id);
    assert_eq!(task["title"], "Test task");
    assert_eq!(task["completed"], false);

    test_app.cleanup().await;
}

#[tokio::test]
async fn get_task_returns_404_for_non_existing_task() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let fake_id = uuid::Uuid::new_v4();

    let response = client
        .get(&format!("{}/tasks/{}", &test_app.address, fake_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_returns_empty_array_when_no_tasks() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/tasks", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let tasks: Vec<serde_json::Value> = response.json().await.expect("Failed to parse response");
    assert_eq!(tasks.len(), 0);

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_returns_all_tasks() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create multiple tasks
    for i in 1..=3 {
        client
            .post(&format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to execute request.");
    }

    // List tasks
    let response = client
        .get(&format!("{}/tasks", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let tasks: Vec<serde_json::Value> = response.json().await.expect("Failed to parse response");
    assert_eq!(tasks.len(), 3);

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_returns_200_for_valid_update() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(&format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Original title"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    let created_task: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse response");
    let task_id = created_task["id"].as_str().unwrap();

    // Update the task
    let response = client
        .put(&format!("{}/tasks/{}", &test_app.address, task_id))
        .json(&json!({
            "title": "Updated title",
            "completed": true
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let updated_task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(updated_task["title"], "Updated title");
    assert_eq!(updated_task["completed"], true);

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_returns_404_for_non_existing_task() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let fake_id = uuid::Uuid::new_v4();

    let response = client
        .put(&format!("{}/tasks/{}", &test_app.address, fake_id))
        .json(&json!({
            "title": "Updated title"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_returns_400_for_empty_title() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(&format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Original title"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    let created_task: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse response");
    let task_id = created_task["id"].as_str().unwrap();

    // Try to update with empty title
    let response = client
        .put(&format!("{}/tasks/{}", &test_app.address, task_id))
        .json(&json!({
            "title": "   "
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_partial_update_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(&format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Original title"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    let created_task: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse response");
    let task_id = created_task["id"].as_str().unwrap();

    // Update only completed status
    let response = client
        .put(&format!("{}/tasks/{}", &test_app.address, task_id))
        .json(&json!({
            "completed": true
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let updated_task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(updated_task["title"], "Original title"); // Title unchanged
    assert_eq!(updated_task["completed"], true); // Completed updated

    test_app.cleanup().await;
}

#[tokio::test]
async fn delete_task_returns_204_for_existing_task() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(&format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Task to delete"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    let created_task: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse response");
    let task_id = created_task["id"].as_str().unwrap();

    // Delete the task
    let response = client
        .delete(&format!("{}/tasks/{}", &test_app.address, task_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify task is deleted
    let get_response = client
        .get(&format!("{}/tasks/{}", &test_app.address, task_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);

    test_app.cleanup().await;
}

#[tokio::test]
async fn delete_task_returns_404_for_non_existing_task() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let fake_id = uuid::Uuid::new_v4();

    let response = client
        .delete(&format!("{}/tasks/{}", &test_app.address, fake_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    test_app.cleanup().await;
}
