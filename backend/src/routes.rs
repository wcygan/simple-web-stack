use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::{json, Value};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    db::AppState,
    errors::AppError,
    models::{CreateTaskPayload, Task, UpdateTaskPayload},
};

pub async fn health_check() -> (StatusCode, Json<Value>) {
    tracing::info!("Health check endpoint called");
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

/// Create a new task
pub async fn create_task(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    tracing::info!("Creating new task with title: {}", payload.title);

    // Validate input
    if payload.title.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Task title cannot be empty".to_string(),
        ));
    }

    let task_id = Uuid::new_v4();

    // Insert the task into the database
    sqlx::query(
        r#"
        INSERT INTO tasks (id, title, completed)
        VALUES (?, ?, FALSE)
        "#,
    )
    .bind(task_id.to_string())
    .bind(payload.title.trim())
    .execute(&app_state.pool)
    .await?;

    // Fetch the created task
    let row = sqlx::query(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(task_id.to_string())
    .fetch_one(&app_state.pool)
    .await?;

    let task = Task {
        id: Uuid::parse_str(row.try_get("id")?).unwrap(),
        title: row.try_get("title")?,
        completed: row.try_get::<i8, _>("completed")? != 0,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    };

    tracing::info!("Task created successfully with id: {}", task_id);
    Ok((StatusCode::CREATED, Json(task)))
}

/// Get a task by ID
pub async fn get_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Task>, AppError> {
    tracing::info!("Fetching task with id: {}", task_id);

    let row = sqlx::query(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(task_id.to_string())
    .fetch_one(&app_state.pool)
    .await?;

    let task = Task {
        id: Uuid::parse_str(row.try_get("id")?).unwrap(),
        title: row.try_get("title")?,
        completed: row.try_get::<i8, _>("completed")? != 0,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    };

    Ok(Json(task))
}

/// List all tasks
pub async fn list_tasks(State(app_state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    tracing::info!("Listing all tasks");

    let rows = sqlx::query(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM tasks
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&app_state.pool)
    .await?;

    let tasks: Vec<Task> = rows
        .into_iter()
        .map(|row| {
            Ok(Task {
                id: Uuid::parse_str(row.try_get("id")?).unwrap(),
                title: row.try_get("title")?,
                completed: row.try_get::<i8, _>("completed")? != 0,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;

    tracing::info!("Found {} tasks", tasks.len());
    Ok(Json(tasks))
}

/// Update a task
pub async fn update_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskPayload>,
) -> Result<Json<Task>, AppError> {
    tracing::info!("Updating task with id: {}", task_id);

    // Check if task exists
    let _ = sqlx::query(r#"SELECT id FROM tasks WHERE id = ?"#)
        .bind(task_id.to_string())
        .fetch_one(&app_state.pool)
        .await?;

    // Build dynamic update query based on provided fields
    let mut query_builder = sqlx::QueryBuilder::new("UPDATE tasks SET ");
    let mut has_updates = false;

    if let Some(title) = &payload.title {
        if title.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Task title cannot be empty".to_string(),
            ));
        }
        query_builder.push("title = ");
        query_builder.push_bind(title.trim());
        has_updates = true;
    }

    if let Some(completed) = payload.completed {
        if has_updates {
            query_builder.push(", ");
        }
        query_builder.push("completed = ");
        query_builder.push_bind(completed);
        has_updates = true;
    }

    if !has_updates {
        return Err(AppError::ValidationError("No fields to update".to_string()));
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(task_id.to_string());

    let query = query_builder.build();
    query.execute(&app_state.pool).await?;

    // Fetch the updated task
    let row = sqlx::query(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(task_id.to_string())
    .fetch_one(&app_state.pool)
    .await?;

    let task = Task {
        id: Uuid::parse_str(row.try_get("id")?).unwrap(),
        title: row.try_get("title")?,
        completed: row.try_get::<i8, _>("completed")? != 0,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    };

    tracing::info!("Task updated successfully");
    Ok(Json(task))
}

/// Delete a task
pub async fn delete_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    tracing::info!("Deleting task with id: {}", task_id);

    let result = sqlx::query(r#"DELETE FROM tasks WHERE id = ?"#)
        .bind(task_id.to_string())
        .execute(&app_state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    tracing::info!("Task deleted successfully");
    Ok(StatusCode::NO_CONTENT)
}

/// Create task routes
pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_task).get(list_tasks))
        .route("/{id}", get(get_task).put(update_task).delete(delete_task))
}
