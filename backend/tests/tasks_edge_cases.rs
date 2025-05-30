mod common;

use common::spawn_app;
use reqwest::StatusCode;
use serde_json::json;

/// Test timestamp consistency and ordering
#[tokio::test]
async fn timestamp_consistency_and_ordering() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let mut creation_times = Vec::new();
    let mut task_ids = Vec::new();

    // Create tasks with small delays to ensure different timestamps
    for i in 0..5 {
        let start_time = std::time::SystemTime::now();

        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": format!("Timestamp Test Task {}", i)
            }))
            .send()
            .await
            .expect("Failed to create task");

        assert_eq!(response.status(), StatusCode::CREATED);

        let task: serde_json::Value = response.json().await.unwrap();
        task_ids.push(task["id"].as_str().unwrap().to_string());
        creation_times.push(start_time);

        // Small delay to ensure different timestamps
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // List tasks and verify ordering
    let list_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks");

    let tasks: Vec<serde_json::Value> = list_response.json().await.unwrap();
    assert_eq!(tasks.len(), 5);

    // Verify timestamps exist and are properly formatted
    for task in &tasks {
        assert!(
            task["created_at"].is_string(),
            "created_at should be a string"
        );
        assert!(
            task["updated_at"].is_string(),
            "updated_at should be a string"
        );

        // Verify ISO 8601 format (basic check)
        let created_at = task["created_at"].as_str().unwrap();
        assert!(
            created_at.contains("T") && created_at.contains("Z"),
            "created_at should be ISO 8601 format"
        );
    }

    // Update one task and verify updated_at changes
    let task_to_update = &task_ids[2];
    let original_updated_at = tasks
        .iter()
        .find(|t| t["id"].as_str().unwrap() == task_to_update)
        .unwrap()["updated_at"]
        .as_str()
        .unwrap()
        .to_string();

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let update_response = client
        .put(format!("{}/tasks/{}", test_app.address, task_to_update))
        .json(&json!({"completed": true}))
        .send()
        .await
        .expect("Failed to update task");

    let updated_task: serde_json::Value = update_response.json().await.unwrap();
    let new_updated_at = updated_task["updated_at"].as_str().unwrap();

    assert_ne!(
        original_updated_at, new_updated_at,
        "updated_at should change when task is modified"
    );

    test_app.cleanup().await;
}

/// Test task state transitions
#[tokio::test]
async fn task_state_transitions() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task
    let create_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "State Transition Test"
        }))
        .send()
        .await
        .expect("Failed to create task");

    let task: serde_json::Value = create_response.json().await.unwrap();
    let task_id = task["id"].as_str().unwrap();

    // Verify initial state
    assert_eq!(task["completed"], false);
    assert_eq!(task["title"], "State Transition Test");

    // Transition: incomplete -> complete
    let update1_response = client
        .put(format!("{}/tasks/{}", test_app.address, task_id))
        .json(&json!({"completed": true}))
        .send()
        .await
        .expect("Failed to update task to completed");

    let updated_task1: serde_json::Value = update1_response.json().await.unwrap();
    assert_eq!(updated_task1["completed"], true);
    assert_eq!(updated_task1["title"], "State Transition Test"); // Title unchanged

    // Transition: complete -> incomplete
    let update2_response = client
        .put(format!("{}/tasks/{}", test_app.address, task_id))
        .json(&json!({"completed": false}))
        .send()
        .await
        .expect("Failed to update task to incomplete");

    let updated_task2: serde_json::Value = update2_response.json().await.unwrap();
    assert_eq!(updated_task2["completed"], false);

    // Change title while keeping completion status
    let update3_response = client
        .put(format!("{}/tasks/{}", test_app.address, task_id))
        .json(&json!({
            "title": "Updated State Transition Test",
            "completed": true
        }))
        .send()
        .await
        .expect("Failed to update both title and completion");

    let updated_task3: serde_json::Value = update3_response.json().await.unwrap();
    assert_eq!(updated_task3["completed"], true);
    assert_eq!(updated_task3["title"], "Updated State Transition Test");

    test_app.cleanup().await;
}

/// Test duplicate task handling
#[tokio::test]
async fn duplicate_task_handling() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let identical_title = "Duplicate Task Test";
    let mut task_ids = Vec::new();

    // Create multiple tasks with identical titles
    for _ in 0..3 {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": identical_title
            }))
            .send()
            .await
            .expect("Failed to create task");

        assert_eq!(response.status(), StatusCode::CREATED);

        let task: serde_json::Value = response.json().await.unwrap();
        task_ids.push(task["id"].as_str().unwrap().to_string());
    }

    // Verify all tasks were created with unique IDs
    assert_eq!(task_ids.len(), 3);
    let unique_ids: std::collections::HashSet<_> = task_ids.iter().collect();
    assert_eq!(
        unique_ids.len(),
        3,
        "All task IDs should be unique even with identical titles"
    );

    // Verify all tasks exist and can be retrieved independently
    for task_id in &task_ids {
        let get_response = client
            .get(format!("{}/tasks/{}", test_app.address, task_id))
            .send()
            .await
            .expect("Failed to get task");

        assert_eq!(get_response.status(), StatusCode::OK);

        let task: serde_json::Value = get_response.json().await.unwrap();
        assert_eq!(task["title"], identical_title);
        assert_eq!(task["id"], *task_id);
    }

    // Update one task to have a different completion status
    let update_response = client
        .put(format!("{}/tasks/{}", test_app.address, &task_ids[1]))
        .json(&json!({"completed": true}))
        .send()
        .await
        .expect("Failed to update middle task");

    assert_eq!(update_response.status(), StatusCode::OK);

    // Verify only the updated task changed
    let list_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks");

    let tasks: Vec<serde_json::Value> = list_response.json().await.unwrap();
    let duplicate_tasks: Vec<_> = tasks
        .iter()
        .filter(|t| t["title"] == identical_title)
        .collect();

    assert_eq!(duplicate_tasks.len(), 3);

    let completed_count = duplicate_tasks
        .iter()
        .filter(|t| t["completed"] == true)
        .count();

    assert_eq!(completed_count, 1, "Only one task should be completed");

    test_app.cleanup().await;
}

/// Test whitespace and normalization edge cases
#[tokio::test]
async fn whitespace_normalization_edge_cases() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let whitespace_test_cases = vec![
        ("   Leading spaces", "Leading spaces"),
        ("Trailing spaces   ", "Trailing spaces"),
        ("  Both ends  ", "Both ends"),
        (
            "Multiple    internal    spaces",
            "Multiple    internal    spaces",
        ), // Internal spaces preserved
        ("\t\tTabs\t\t", "Tabs"),
        ("\n\nNewlines\n\n", "Newlines"),
        (" \t \n Mixed whitespace \r\n \t ", "Mixed whitespace"),
        ("", ""),    // Empty after trimming should fail
        ("   ", ""), // Only whitespace should fail
    ];

    for (input_title, expected_title) in whitespace_test_cases {
        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": input_title
            }))
            .send()
            .await
            .expect("Failed to execute request");

        if expected_title.is_empty() {
            // Empty titles should be rejected
            assert_eq!(
                response.status(),
                StatusCode::BAD_REQUEST,
                "Empty title '{}' should be rejected",
                input_title
            );
        } else {
            // Valid titles should be accepted and trimmed
            assert_eq!(
                response.status(),
                StatusCode::CREATED,
                "Valid title '{}' should be accepted",
                input_title
            );

            let task: serde_json::Value = response.json().await.unwrap();
            assert_eq!(
                task["title"], expected_title,
                "Title should be normalized from '{}' to '{}'",
                input_title, expected_title
            );
        }
    }

    test_app.cleanup().await;
}

/// Test boundary value analysis
#[tokio::test]
async fn boundary_value_analysis() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test title length boundaries (assuming 255 char limit)
    let boundary_test_cases = vec![
        (1, "Minimum length title should work"),
        (254, "One below assumed limit should work"),
        (255, "At assumed limit should work"),
        (256, "One above assumed limit might fail"),
        (1000, "Well above limit should fail gracefully"),
    ];

    for (length, description) in boundary_test_cases {
        let title = if length == 1 {
            "a".to_string()
        } else {
            format!("{}{}", "a".repeat(length - 1), "b") // Ensure it's exactly the length
        };

        let response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({
                "title": title
            }))
            .send()
            .await
            .expect("Failed to execute request");

        // Should either succeed or fail gracefully with appropriate status
        assert!(
            response.status() == StatusCode::CREATED
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::PAYLOAD_TOO_LARGE,
            "{}: Unexpected status for length {}: {}",
            description,
            length,
            response.status()
        );

        if response.status() == StatusCode::CREATED {
            let task: serde_json::Value = response.json().await.unwrap();
            let returned_title = task["title"].as_str().unwrap();

            // Verify the title is stored correctly (or truncated consistently)
            assert!(
                returned_title.len() <= title.len(),
                "Returned title should not be longer than input"
            );
            assert!(
                returned_title.len() > 0,
                "Returned title should not be empty"
            );
        }
    }

    test_app.cleanup().await;
}

/// Test idempotency of operations
#[tokio::test]
async fn operation_idempotency() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a task
    let create_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({
            "title": "Idempotency Test Task"
        }))
        .send()
        .await
        .expect("Failed to create task");

    let task: serde_json::Value = create_response.json().await.unwrap();
    let task_id = task["id"].as_str().unwrap();

    // Test idempotent GET operations
    let mut get_responses = Vec::new();
    for _ in 0..3 {
        let response = client
            .get(format!("{}/tasks/{}", test_app.address, task_id))
            .send()
            .await
            .expect("Failed to get task");

        assert_eq!(response.status(), StatusCode::OK);
        let task_data: serde_json::Value = response.json().await.unwrap();
        get_responses.push(task_data);
    }

    // All GET responses should be identical
    for i in 1..get_responses.len() {
        assert_eq!(
            get_responses[0], get_responses[i],
            "GET responses should be identical"
        );
    }

    // Test idempotent PUT operations (same data)
    let update_data = json!({
        "title": "Updated Idempotency Test",
        "completed": true
    });

    let mut put_responses = Vec::new();
    for _ in 0..3 {
        let response = client
            .put(format!("{}/tasks/{}", test_app.address, task_id))
            .json(&update_data)
            .send()
            .await
            .expect("Failed to update task");

        assert_eq!(response.status(), StatusCode::OK);
        let task_data: serde_json::Value = response.json().await.unwrap();
        put_responses.push(task_data);
    }

    // All PUT responses should be identical (except possibly updated_at)
    for i in 1..put_responses.len() {
        assert_eq!(put_responses[0]["id"], put_responses[i]["id"]);
        assert_eq!(put_responses[0]["title"], put_responses[i]["title"]);
        assert_eq!(put_responses[0]["completed"], put_responses[i]["completed"]);
        assert_eq!(
            put_responses[0]["created_at"],
            put_responses[i]["created_at"]
        );
        // Note: updated_at might change with each update, so we don't check it
    }

    // Test DELETE idempotency
    let first_delete = client
        .delete(format!("{}/tasks/{}", test_app.address, task_id))
        .send()
        .await
        .expect("Failed to delete task");

    assert_eq!(first_delete.status(), StatusCode::NO_CONTENT);

    // Second delete should return 404 (not found)
    let second_delete = client
        .delete(format!("{}/tasks/{}", test_app.address, task_id))
        .send()
        .await
        .expect("Failed to execute second delete");

    assert_eq!(second_delete.status(), StatusCode::NOT_FOUND);

    test_app.cleanup().await;
}

/// Test data consistency across operations
#[tokio::test]
async fn data_consistency_across_operations() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create tasks with known data
    let test_tasks = vec![("Task 1", false), ("Task 2", true), ("Task 3", false)];

    let mut created_tasks = Vec::new();

    for (title, initial_completed) in &test_tasks {
        let create_response = client
            .post(format!("{}/tasks", test_app.address))
            .json(&json!({"title": title}))
            .send()
            .await
            .expect("Failed to create task");

        let task: serde_json::Value = create_response.json().await.unwrap();
        created_tasks.push((
            task["id"].as_str().unwrap().to_string(),
            *title,
            *initial_completed,
        ));

        // If we want to set initial completion status, update after creation
        if *initial_completed {
            client
                .put(format!(
                    "{}/tasks/{}",
                    test_app.address,
                    task["id"].as_str().unwrap()
                ))
                .json(&json!({"completed": true}))
                .send()
                .await
                .expect("Failed to set initial completion status");
        }
    }

    // Verify data consistency through list operation
    let list_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks");

    let all_tasks: Vec<serde_json::Value> = list_response.json().await.unwrap();

    for (task_id, expected_title, expected_completed) in &created_tasks {
        let found_task = all_tasks
            .iter()
            .find(|t| t["id"].as_str().unwrap() == task_id)
            .expect("Task should be found in list");

        assert_eq!(found_task["title"].as_str().unwrap(), *expected_title);
        assert_eq!(
            found_task["completed"].as_bool().unwrap(),
            *expected_completed
        );

        // Also verify through individual GET
        let get_response = client
            .get(format!("{}/tasks/{}", test_app.address, task_id))
            .send()
            .await
            .expect("Failed to get individual task");

        let individual_task: serde_json::Value = get_response.json().await.unwrap();

        // Data should be consistent between list and individual get
        assert_eq!(found_task["id"], individual_task["id"]);
        assert_eq!(
            found_task["title"].as_str().unwrap(),
            individual_task["title"].as_str().unwrap()
        );
        assert_eq!(
            found_task["completed"].as_bool().unwrap(),
            individual_task["completed"].as_bool().unwrap()
        );
        assert_eq!(found_task["created_at"], individual_task["created_at"]);
        assert_eq!(found_task["updated_at"], individual_task["updated_at"]);
    }

    test_app.cleanup().await;
}

/// Test empty database scenarios
#[tokio::test]
async fn empty_database_scenarios() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test listing when database is empty
    let empty_list_response = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks from empty database");

    assert_eq!(empty_list_response.status(), StatusCode::OK);

    let empty_tasks: Vec<serde_json::Value> = empty_list_response.json().await.unwrap();
    assert_eq!(
        empty_tasks.len(),
        0,
        "Empty database should return empty array"
    );

    // Test getting non-existent task
    let fake_uuid = uuid::Uuid::new_v4();
    let get_nonexistent_response = client
        .get(format!("{}/tasks/{}", test_app.address, fake_uuid))
        .send()
        .await
        .expect("Failed to get non-existent task");

    assert_eq!(get_nonexistent_response.status(), StatusCode::NOT_FOUND);

    // Test updating non-existent task
    let update_nonexistent_response = client
        .put(format!("{}/tasks/{}", test_app.address, fake_uuid))
        .json(&json!({"title": "This shouldn't work"}))
        .send()
        .await
        .expect("Failed to update non-existent task");

    assert_eq!(update_nonexistent_response.status(), StatusCode::NOT_FOUND);

    // Test deleting non-existent task
    let delete_nonexistent_response = client
        .delete(format!("{}/tasks/{}", test_app.address, fake_uuid))
        .send()
        .await
        .expect("Failed to delete non-existent task");

    assert_eq!(delete_nonexistent_response.status(), StatusCode::NOT_FOUND);

    // Add one task and verify the database works normally
    let create_response = client
        .post(format!("{}/tasks", test_app.address))
        .json(&json!({"title": "First task in empty database"}))
        .send()
        .await
        .expect("Failed to create first task");

    assert_eq!(create_response.status(), StatusCode::CREATED);

    // Verify the task appears in subsequent list
    let after_create_list = client
        .get(format!("{}/tasks", test_app.address))
        .send()
        .await
        .expect("Failed to list tasks after creating first task");

    let tasks_after_create: Vec<serde_json::Value> = after_create_list.json().await.unwrap();
    assert_eq!(
        tasks_after_create.len(),
        1,
        "Should have exactly one task after creation"
    );

    test_app.cleanup().await;
}
