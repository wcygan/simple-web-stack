mod common;

use common::spawn_app;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn create_task_returns_201_for_valid_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/tasks", &test_app.address))
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
        .post(format!("{}/tasks", &test_app.address))
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
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({}))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    test_app.cleanup().await;
}

#[tokio::test]
async fn create_task_handles_title_with_leading_trailing_spaces() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "  A task with spaces  "
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);
    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    // Assuming the backend trims whitespace from titles upon creation or before storing.
    // If not, this assertion should be "  A task with spaces  ".
    assert_eq!(task["title"], "A task with spaces");
    assert_eq!(task["completed"], false);

    test_app.cleanup().await;
}

#[tokio::test]
async fn create_task_handles_title_with_special_characters() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let special_title = "A task with !@#$%^&*()_+-=[]{};':\",./<>?";

    let response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": special_title
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);
    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(task["title"], special_title);
    assert_eq!(task["completed"], false);

    test_app.cleanup().await;
}

#[tokio::test]
async fn create_task_handles_max_length_title() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    // Assuming a max title length of 255 for this test.
    // The actual limit should be based on database schema or application constraints.
    let max_length_title = "a".repeat(255);

    let response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": max_length_title
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);
    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(task["title"], max_length_title);
    assert_eq!(task["completed"], false);

    // Optional: Test for title slightly over max length if there's a specific error for it.
    // let overly_long_title = "a".repeat(256);
    // let response_over_limit = client
    //     .post(&format!("{}/tasks", &test_app.address))
    //     .json(&json!({
    //         "title": overly_long_title
    //     }))
    //     .send()
    //     .await
    //     .expect("Failed to execute request.");
    // assert_eq!(response_over_limit.status(), StatusCode::BAD_REQUEST); // Or appropriate error

    test_app.cleanup().await;
}

#[tokio::test]
async fn create_task_ignores_explicit_id_in_request() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let provided_id = uuid::Uuid::new_v4().to_string();

    let response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({
            "id": provided_id,
            "title": "Task with explicit ID"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);
    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_ne!(task["id"], provided_id); // Server should generate its own ID
    assert!(task["id"].is_string());
    assert_eq!(task["title"], "Task with explicit ID");

    test_app.cleanup().await;
}

#[tokio::test]
async fn create_task_ignores_explicit_completed_status_in_request() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Task with explicit completed status",
            "completed": true // Attempt to create as completed
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);
    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(task["completed"], false); // Should default to false
    assert_eq!(task["title"], "Task with explicit completed status");

    test_app.cleanup().await;
}

#[tokio::test]
async fn get_task_returns_200_for_existing_task() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
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
        .get(format!("{}/tasks/{}", &test_app.address, task_id))
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
        .get(format!("{}/tasks/{}", &test_app.address, fake_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    test_app.cleanup().await;
}

#[tokio::test]
async fn get_task_returns_400_for_invalid_uuid_format() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let invalid_uuid = "not-a-uuid";

    let response = client
        .get(format!("{}/tasks/{}", &test_app.address, invalid_uuid))
        .send()
        .await
        .expect("Failed to execute request.");

    // Depending on router/handler logic, this might be a 400 Bad Request
    // or a 404 Not Found if the path matching fails before validation.
    // Axum typically returns 404 for malformed path parameters that don't match a route.
    // If a specific handler parses the UUID and fails, it might return 400.
    // Let's assume 404 for now if the route doesn't match due to invalid param format.
    // If it's a 400 due to Uuid::parse_str failing deeper in the handler, adjust this.
    assert!(
        response.status() == StatusCode::BAD_REQUEST || response.status() == StatusCode::NOT_FOUND
    );

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_returns_empty_array_when_no_tasks() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/tasks", &test_app.address))
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
    let mut created_task_ids = Vec::new();
    for i in 1..=3 {
        let create_response = client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": format!("Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to execute request.");
        let task: serde_json::Value = create_response.json().await.unwrap();
        created_task_ids.push(task["id"].as_str().unwrap().to_string());
    }

    // List tasks
    let response = client
        .get(format!("{}/tasks", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let tasks: Vec<serde_json::Value> = response.json().await.expect("Failed to parse response");
    assert_eq!(tasks.len(), 3);

    // Verify all created tasks are present
    for task_id in created_task_ids {
        assert!(tasks.iter().any(|t| t["id"].as_str().unwrap() == task_id));
    }

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_returns_many_tasks() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let num_tasks_to_create = 100;
    let mut created_task_titles = Vec::new();

    for i in 0..num_tasks_to_create {
        let title = format!("Bulk Task {}", i);
        created_task_titles.push(title.clone());
        client
            .post(format!("{}/tasks", &test_app.address))
            .json(&json!({
                "title": title
            }))
            .send()
            .await
            .expect("Failed to execute request for bulk task creation");
    }

    let response = client
        .get(format!("{}/tasks", &test_app.address))
        .send()
        .await
        .expect("Failed to execute list tasks request.");

    assert_eq!(response.status(), StatusCode::OK);
    let tasks: Vec<serde_json::Value> = response
        .json()
        .await
        .expect("Failed to parse list tasks response");

    assert_eq!(tasks.len(), num_tasks_to_create);

    // Optionally, verify some of the tasks to ensure data integrity
    for title in created_task_titles {
        assert!(tasks
            .iter()
            .any(|task| task["title"].as_str().unwrap() == title));
    }

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_returns_200_for_valid_update() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
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
        .put(format!("{}/tasks/{}", &test_app.address, task_id))
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
        .put(format!("{}/tasks/{}", &test_app.address, fake_id))
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
        .post(format!("{}/tasks", &test_app.address))
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
        .put(format!("{}/tasks/{}", &test_app.address, task_id))
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
        .post(format!("{}/tasks", &test_app.address))
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
        .put(format!("{}/tasks/{}", &test_app.address, task_id))
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
async fn update_task_handles_title_with_leading_trailing_spaces() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({"title": "Initial Title"}))
        .send()
        .await
        .unwrap();
    let task_id = create_response.json::<serde_json::Value>().await.unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();

    let response = client
        .put(format!("{}/tasks/{}", &test_app.address, &task_id))
        .json(&json!({
            "title": "  Updated Title With Spaces  "
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    // Assuming title is trimmed upon update
    assert_eq!(task["title"], "Updated Title With Spaces");

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_handles_title_with_special_characters() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let special_title = "Updated Task !@#$%^&*()_+-=[]{};':\",./<>?";

    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({"title": "Initial Title For Special Chars"}))
        .send()
        .await
        .unwrap();
    let task_id = create_response.json::<serde_json::Value>().await.unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();

    let response = client
        .put(format!("{}/tasks/{}", &test_app.address, &task_id))
        .json(&json!({
            "title": special_title
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(task["title"], special_title);

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_returns_400_for_invalid_uuid_format() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let invalid_uuid = "not-a-uuid";

    let response = client
        .put(format!("{}/tasks/{}", &test_app.address, invalid_uuid))
        .json(&json!({
            "title": "Update with invalid id"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    // Similar to GET, this could be 404 or 400. Let's assume 404 for path matching failure.
    assert!(
        response.status() == StatusCode::BAD_REQUEST || response.status() == StatusCode::NOT_FOUND
    );

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_returns_422_for_empty_json_body() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({"title": "Task to update with empty body"}))
        .send()
        .await
        .unwrap();
    let task_id = create_response.json::<serde_json::Value>().await.unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();

    let response = client
        .put(format!("{}/tasks/{}", &test_app.address, &task_id))
        .header("Content-Type", "application/json") // Ensure content type is set
        .body("{}") // Send an empty JSON object
        .send()
        .await
        .expect("Failed to execute request.");

    // Expecting Unprocessable Entity because the server expects some fields for an update,
    // even if they are optional. An empty JSON might not be a valid partial update.
    // Or, if the server allows it and treats it as "no changes", it could be 200.
    // For now, let's assume 422 if no recognized fields are provided for an update.
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    test_app.cleanup().await;
}

#[tokio::test]
async fn update_task_ignores_extra_fields_in_request() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({"title": "Task for extra fields test"}))
        .send()
        .await
        .unwrap();
    let task_id = create_response.json::<serde_json::Value>().await.unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();

    let response = client
        .put(format!("{}/tasks/{}", &test_app.address, &task_id))
        .json(&json!({
            "title": "Updated Title With Extra",
            "completed": true,
            "extra_field": "should be ignored",
            "another_unknown": 123
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    let task: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(task["title"], "Updated Title With Extra");
    assert_eq!(task["completed"], true);
    assert!(task.get("extra_field").is_none());
    assert!(task.get("another_unknown").is_none());

    test_app.cleanup().await;
}

#[tokio::test]
async fn delete_task_returns_204_for_existing_task() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
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
        .delete(format!("{}/tasks/{}", &test_app.address, task_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify task is deleted
    let get_response = client
        .get(format!("{}/tasks/{}", &test_app.address, task_id))
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
        .delete(format!("{}/tasks/{}", &test_app.address, fake_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    test_app.cleanup().await;
}

#[tokio::test]
async fn delete_task_returns_400_for_invalid_uuid_format() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let invalid_uuid = "not-a-uuid";

    let response = client
        .delete(format!("{}/tasks/{}", &test_app.address, invalid_uuid))
        .send()
        .await
        .expect("Failed to execute request.");

    // Expecting 404 if route matching fails due to bad param format, or 400 if it hits a parse error deeper.
    assert!(
        response.status() == StatusCode::BAD_REQUEST || response.status() == StatusCode::NOT_FOUND
    );

    test_app.cleanup().await;
}

// General API Behavior Tests
#[tokio::test]
async fn post_tasks_returns_json_content_type() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Content-Type Test"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    test_app.cleanup().await;
}

#[tokio::test]
async fn get_task_returns_json_content_type() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Content-Type Test GET"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    let created_task: serde_json::Value = create_response.json().await.unwrap();
    let task_id = created_task["id"].as_str().unwrap();

    let response = client
        .get(format!("{}/tasks/{}", &test_app.address, task_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    test_app.cleanup().await;
}

#[tokio::test]
async fn list_tasks_returns_json_content_type() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/tasks", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    test_app.cleanup().await;
}

#[tokio::test]
async fn put_task_returns_json_content_type() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task first
    let create_response = client
        .post(format!("{}/tasks", &test_app.address))
        .json(&json!({
            "title": "Content-Type Test PUT"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    let created_task: serde_json::Value = create_response.json().await.unwrap();
    let task_id = created_task["id"].as_str().unwrap();

    let response = client
        .put(format!("{}/tasks/{}", &test_app.address, task_id))
        .json(&json!({
            "title": "Updated Content-Type Test PUT",
            "completed": true
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    test_app.cleanup().await;
}
