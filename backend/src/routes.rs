use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    auth::{hash_password, verify_password, generate_session_token, hash_session_token},
    db::AppState,
    errors::AppError,
    models::{
        AuthResponse, CreateTaskPayload, LoginPayload, PaginatedResponse, PaginationMeta, 
        RegisterPayload, Task, TaskQueryParams, UpdateTaskPayload, UserResponse
    },
    middleware::AuthUser,
};

/// Maximum allowed title length
const MAX_TITLE_LENGTH: usize = 255;

/// Helper function to validate task title
fn validate_title(title: &str) -> Result<String, AppError> {
    let trimmed = title.trim();

    if trimmed.is_empty() {
        return Err(AppError::ValidationError(
            "Task title cannot be empty".to_string(),
        ));
    }

    if trimmed.len() > MAX_TITLE_LENGTH {
        return Err(AppError::ValidationError(format!(
            "Task title cannot exceed {} characters",
            MAX_TITLE_LENGTH
        )));
    }

    Ok(trimmed.to_string())
}

/// Helper function to convert a database row to a Task
fn row_to_task(row: &MySqlRow) -> Result<Task, sqlx::Error> {
    Ok(Task {
        id: Uuid::parse_str(row.try_get("id")?).unwrap(),
        title: row.try_get("title")?,
        completed: row.try_get::<i8, _>("completed")? != 0,
        user_id: Uuid::parse_str(row.try_get("user_id")?).unwrap(),
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

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
    let validated_title = validate_title(&payload.title)?;

    let task_id = Uuid::new_v4();

    // Insert the task into the database
    sqlx::query(
        r#"
        INSERT INTO tasks (id, title, completed, user_id)
        VALUES (?, ?, FALSE, ?)
        "#,
    )
    .bind(task_id.to_string())
    .bind(&validated_title)
    .bind("00000000-0000-0000-0000-000000000000")
    .execute(&app_state.pool)
    .await?;

    // Fetch the created task
    let row = sqlx::query(
        r#"
        SELECT id, title, completed, user_id, created_at, updated_at
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(task_id.to_string())
    .fetch_one(&app_state.pool)
    .await?;

    let task = row_to_task(&row)?;

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
        SELECT id, title, completed, user_id, created_at, updated_at
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(task_id.to_string())
    .fetch_one(&app_state.pool)
    .await?;

    let task = row_to_task(&row)?;

    Ok(Json(task))
}

/// List all tasks with pagination and search support
pub async fn list_tasks(
    State(app_state): State<AppState>,
    Query(mut query_params): Query<TaskQueryParams>,
) -> Result<Json<PaginatedResponse<Task>>, AppError> {
    tracing::info!("Listing tasks with search/pagination: page={}, page_size={}, q={:?}, status={:?}", 
        query_params.pagination.page, 
        query_params.pagination.page_size,
        query_params.search.q,
        query_params.search.status
    );

    // Validate and adjust pagination parameters
    query_params.pagination.validate();

    // Get total count with search filters
    let total_items = crate::db::count_tasks_with_search(
        &app_state.pool,
        query_params.search.q.as_deref(),
        query_params.search.status.as_deref(),
    ).await?;

    // Get paginated tasks with search filters
    let tasks = crate::db::get_paginated_tasks_with_search(
        &app_state.pool,
        query_params.pagination.offset(),
        query_params.pagination.limit(),
        query_params.pagination.sort_by.as_deref(),
        query_params.pagination.sort_order.as_ref(),
        query_params.search.q.as_deref(),
        query_params.search.status.as_deref(),
    ).await?;

    // Create pagination metadata
    let pagination = PaginationMeta::new(query_params.pagination.page, query_params.pagination.page_size, total_items);

    let response = PaginatedResponse {
        data: tasks,
        pagination,
    };

    tracing::info!("Found {} tasks (total: {}, page: {}/{})", 
        response.data.len(), 
        total_items, 
        query_params.pagination.page, 
        response.pagination.total_pages
    );

    Ok(Json(response))
}

/// List all tasks (legacy endpoint for backwards compatibility)
pub async fn list_all_tasks(State(app_state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    tracing::info!("Listing all tasks (legacy endpoint)");

    let rows = sqlx::query(
        r#"
        SELECT id, title, completed, user_id, created_at, updated_at
        FROM tasks
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&app_state.pool)
    .await?;

    let tasks: Vec<Task> = rows
        .iter()
        .map(row_to_task)
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

    // Validate title if provided
    let validated_title = if let Some(title) = &payload.title {
        Some(validate_title(title)?)
    } else {
        None
    };

    // Build dynamic update query based on provided fields
    let mut query_builder = sqlx::QueryBuilder::new("UPDATE tasks SET ");
    let mut has_updates = false;

    if let Some(title) = &validated_title {
        query_builder.push("title = ");
        query_builder.push_bind(title);
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
        return Err(AppError::NoFieldsToUpdate);
    }

    // Always update the updated_at timestamp to ensure it changes
    if has_updates {
        query_builder.push(", ");
    }
    query_builder.push("updated_at = CURRENT_TIMESTAMP(6)");

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(task_id.to_string());

    let query = query_builder.build();
    query.execute(&app_state.pool).await?;

    // Fetch the updated task
    let row = sqlx::query(
        r#"
        SELECT id, title, completed, user_id, created_at, updated_at
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(task_id.to_string())
    .fetch_one(&app_state.pool)
    .await?;

    let task = row_to_task(&row)?;

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

/// Register a new user
pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>, AppError> {
    tracing::info!("Registering new user with email: {}", payload.email);

    // Validate email format (basic validation)
    if !payload.email.contains('@') || payload.email.len() > 255 {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }

    // Validate password strength
    if payload.password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters long".to_string(),
        ));
    }

    // Check if user already exists
    let existing_user = crate::db::get_user_by_email(&app_state.pool, &payload.email).await?;
    if existing_user.is_some() {
        return Err(AppError::ValidationError(
            "User with this email already exists".to_string(),
        ));
    }

    // Hash the password
    let password_hash = hash_password(&payload.password)
        .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))?;

    // Create the user
    let user_id = Uuid::new_v4();
    let user = crate::db::create_user(&app_state.pool, user_id, &payload.email, &password_hash).await?;

    // Create a session
    let session_id = Uuid::new_v4();
    let session_token = generate_session_token();
    let token_hash = hash_session_token(&session_token);
    let expires_at = chrono::Utc::now() + app_state.auth_service.get_token_expiry_duration();

    let _session = crate::db::create_session(
        &app_state.pool,
        session_id,
        user_id,
        &token_hash,
        expires_at,
    ).await?;

    // Generate JWT
    let jwt_token = app_state
        .auth_service
        .generate_token(user_id, session_id)
        .map_err(|_| AppError::InternalServerError("Failed to generate token".to_string()))?;

    tracing::info!("User registered successfully");
    Ok(Json(AuthResponse {
        token: jwt_token,
        user: UserResponse::from(user),
    }))
}

/// Login a user
pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, AppError> {
    tracing::info!("User login attempt for email: {}", payload.email);

    // Get user by email
    let user = crate::db::get_user_by_email(&app_state.pool, &payload.email)
        .await?
        .ok_or(AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    let password_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|_| AppError::InternalServerError("Failed to verify password".to_string()))?;

    if !password_valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Create a new session
    let session_id = Uuid::new_v4();
    let session_token = generate_session_token();
    let token_hash = hash_session_token(&session_token);
    let expires_at = chrono::Utc::now() + app_state.auth_service.get_token_expiry_duration();

    let _session = crate::db::create_session(
        &app_state.pool,
        session_id,
        user.id,
        &token_hash,
        expires_at,
    ).await?;

    // Generate JWT
    let jwt_token = app_state
        .auth_service
        .generate_token(user.id, session_id)
        .map_err(|_| AppError::InternalServerError("Failed to generate token".to_string()))?;

    tracing::info!("User logged in successfully");
    Ok(Json(AuthResponse {
        token: jwt_token,
        user: UserResponse::from(user),
    }))
}

/// Logout a user (requires auth in real implementation)
pub async fn logout(
    State(_app_state): State<AppState>,
) -> Result<StatusCode, AppError> {
    tracing::info!("User logout endpoint called");

    // TODO: Extract user from auth middleware and delete session
    // crate::db::delete_session(&app_state.pool, auth_user.session_id).await?;

    tracing::info!("User logged out successfully");
    Ok(StatusCode::NO_CONTENT)
}

/// Get current user profile (requires auth in real implementation)
pub async fn get_profile(
    State(_app_state): State<AppState>,
) -> Result<Json<UserResponse>, AppError> {
    tracing::info!("Get profile endpoint called");

    // TODO: Extract user from auth middleware and return user data
    // let user = crate::db::get_user_by_id(&app_state.pool, auth_user.user_id).await?;
    // Ok(Json(UserResponse::from(user)))
    
    Err(AppError::Unauthorized("Authentication middleware not fully implemented".to_string()))
}

/// Create task routes
pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_task).get(list_tasks))
        .route("/{id}", get(get_task).put(update_task).delete(delete_task))
}

/// Create public auth routes (no authentication required)
pub fn public_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

/// Create protected auth routes (authentication required)
pub fn protected_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
        .route("/profile", get(get_profile))
}
