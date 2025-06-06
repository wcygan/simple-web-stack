use sqlx::{mysql::MySqlPoolOptions, MySqlPool, Row};
use std::time::Duration;

/// Application state that will be shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub auth_service: crate::auth::AuthService,
}

/// Create a MySQL connection pool with the given database URL
pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    tracing::info!("Creating database connection pool");

    MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

/// Initialize the database schema
pub async fn init_db(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    tracing::info!("Initializing database schema");

    // Create the users table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id CHAR(36) PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMP(6) DEFAULT CURRENT_TIMESTAMP(6),
            updated_at TIMESTAMP(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
            INDEX idx_users_email (email)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create the sessions table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id CHAR(36) PRIMARY KEY,
            user_id CHAR(36) NOT NULL,
            token_hash VARCHAR(255) NOT NULL,
            expires_at TIMESTAMP(6) NOT NULL,
            created_at TIMESTAMP(6) DEFAULT CURRENT_TIMESTAMP(6),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            INDEX idx_sessions_token_hash (token_hash),
            INDEX idx_sessions_user_id (user_id),
            INDEX idx_sessions_expires_at (expires_at)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create the tasks table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id CHAR(36) PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            user_id CHAR(36) NOT NULL,
            created_at TIMESTAMP(6) DEFAULT CURRENT_TIMESTAMP(6),
            updated_at TIMESTAMP(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            INDEX idx_tasks_user_id (user_id),
            INDEX idx_tasks_created_at (created_at),
            INDEX idx_tasks_completed (completed)
        )
        "#,
    )
    .execute(pool)
    .await?;

    tracing::info!("Database schema initialized successfully");
    Ok(())
}

/// Count total number of tasks with optional filtering
pub async fn count_tasks(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM tasks")
        .fetch_one(pool)
        .await?;

    let count: i64 = row.get("count");
    Ok(count as u64)
}

/// Count tasks with search and status filtering
pub async fn count_tasks_with_search(
    pool: &MySqlPool,
    search_query: Option<&str>,
    status_filter: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut query_str = String::from("SELECT COUNT(*) as count FROM tasks WHERE 1=1");
    let mut bindings = Vec::new();

    // Add search condition
    if let Some(query) = search_query {
        if !query.trim().is_empty() {
            query_str.push_str(" AND title LIKE ?");
            bindings.push(format!("%{}%", query.trim()));
        }
    }

    // Add status filter
    match status_filter {
        Some("completed") => {
            query_str.push_str(" AND completed = 1");
        }
        Some("pending") => {
            query_str.push_str(" AND completed = 0");
        }
        _ => {} // "all" or None - no additional filter
    }

    let mut query = sqlx::query(&query_str);
    for binding in bindings {
        query = query.bind(binding);
    }

    let row = query.fetch_one(pool).await?;
    let count: i64 = row.get("count");
    Ok(count as u64)
}

/// Get paginated tasks with optional sorting
pub async fn get_paginated_tasks(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
    sort_by: Option<&str>,
    sort_order: Option<&crate::models::SortOrder>,
) -> Result<Vec<crate::models::Task>, sqlx::Error> {
    use crate::models::SortOrder;
    
    // Validate sort column to prevent SQL injection
    let sort_column = match sort_by {
        Some("title") => "title",
        Some("completed") => "completed",
        Some("updated_at") => "updated_at",
        _ => "created_at", // default
    };
    
    let order = match sort_order {
        Some(SortOrder::Asc) => "ASC",
        _ => "DESC", // default to DESC
    };
    
    // Build the query with proper parameter binding for LIMIT and OFFSET
    let query_str = format!(
        "SELECT id, title, completed, user_id, created_at, updated_at FROM tasks ORDER BY {} {} LIMIT ? OFFSET ?",
        sort_column, order
    );
    
    let rows = sqlx::query(&query_str)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

    let tasks: Vec<crate::models::Task> = rows
        .iter()
        .map(|row| {
            use sqlx::Row;
            crate::models::Task {
                id: uuid::Uuid::parse_str(row.get("id")).unwrap(),
                title: row.get("title"),
                completed: row.get::<i8, _>("completed") != 0,
                user_id: uuid::Uuid::parse_str(row.get("user_id")).unwrap(),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        })
        .collect();

    Ok(tasks)
}

/// Get paginated tasks with search and status filtering
pub async fn get_paginated_tasks_with_search(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
    sort_by: Option<&str>,
    sort_order: Option<&crate::models::SortOrder>,
    search_query: Option<&str>,
    status_filter: Option<&str>,
) -> Result<Vec<crate::models::Task>, sqlx::Error> {
    use crate::models::SortOrder;
    
    // Validate sort column to prevent SQL injection
    let sort_column = match sort_by {
        Some("title") => "title",
        Some("completed") => "completed",
        Some("updated_at") => "updated_at",
        _ => "created_at", // default
    };
    
    let order = match sort_order {
        Some(SortOrder::Asc) => "ASC",
        _ => "DESC", // default to DESC
    };
    
    // Build the base query
    let mut query_str = String::from("SELECT id, title, completed, user_id, created_at, updated_at FROM tasks WHERE 1=1");
    let mut bindings = Vec::new();

    // Add search condition
    if let Some(query) = search_query {
        if !query.trim().is_empty() {
            query_str.push_str(" AND title LIKE ?");
            bindings.push(format!("%{}%", query.trim()));
        }
    }

    // Add status filter
    match status_filter {
        Some("completed") => {
            query_str.push_str(" AND completed = 1");
        }
        Some("pending") => {
            query_str.push_str(" AND completed = 0");
        }
        _ => {} // "all" or None - no additional filter
    }

    // Add ordering and pagination
    query_str.push_str(&format!(" ORDER BY {} {} LIMIT ? OFFSET ?", sort_column, order));

    // Build query with all bindings
    let mut query = sqlx::query(&query_str);
    for binding in bindings {
        query = query.bind(binding);
    }
    query = query.bind(limit as i64).bind(offset as i64);

    let rows = query.fetch_all(pool).await?;

    let tasks: Vec<crate::models::Task> = rows
        .iter()
        .map(|row| {
            use sqlx::Row;
            crate::models::Task {
                id: uuid::Uuid::parse_str(row.get("id")).unwrap(),
                title: row.get("title"),
                completed: row.get::<i8, _>("completed") != 0,
                user_id: uuid::Uuid::parse_str(row.get("user_id")).unwrap(),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        })
        .collect();

    Ok(tasks)
}

// Authentication Database Functions

/// Create a new user
pub async fn create_user(
    pool: &MySqlPool,
    id: uuid::Uuid,
    email: &str,
    password_hash: &str,
) -> Result<crate::models::User, sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(id.to_string())
    .bind(email)
    .bind(password_hash)
    .execute(pool)
    .await?;

    // Return the created user
    get_user_by_id(pool, id).await
}

/// Get user by ID
pub async fn get_user_by_id(
    pool: &MySqlPool,
    user_id: uuid::Uuid,
) -> Result<crate::models::User, sqlx::Error> {
    sqlx::query_as::<_, crate::models::User>(
        "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = ?"
    )
    .bind(user_id.to_string())
    .fetch_one(pool)
    .await
}

/// Get user by email
pub async fn get_user_by_email(
    pool: &MySqlPool,
    email: &str,
) -> Result<Option<crate::models::User>, sqlx::Error> {
    let result = sqlx::query_as::<_, crate::models::User>(
        "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = ?"
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

/// Create a new session
pub async fn create_session(
    pool: &MySqlPool,
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    token_hash: &str,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<crate::models::Session, sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO sessions (id, user_id, token_hash, expires_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(id.to_string())
    .bind(user_id.to_string())
    .bind(token_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    // Return the created session
    get_session_by_id(pool, id).await
}

/// Get session by ID
pub async fn get_session_by_id(
    pool: &MySqlPool,
    session_id: uuid::Uuid,
) -> Result<crate::models::Session, sqlx::Error> {
    sqlx::query_as::<_, crate::models::Session>(
        "SELECT id, user_id, token_hash, expires_at, created_at FROM sessions WHERE id = ?"
    )
    .bind(session_id.to_string())
    .fetch_one(pool)
    .await
}

/// Get session by token hash
pub async fn get_session_by_token(
    pool: &MySqlPool,
    token_hash: &str,
) -> Result<Option<crate::models::Session>, sqlx::Error> {
    let result = sqlx::query_as::<_, crate::models::Session>(
        "SELECT id, user_id, token_hash, expires_at, created_at FROM sessions WHERE token_hash = ? AND expires_at > NOW()"
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

/// Delete a session
pub async fn delete_session(
    pool: &MySqlPool,
    session_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(session_id.to_string())
        .execute(pool)
        .await?;

    Ok(())
}

/// Delete expired sessions
pub async fn cleanup_expired_sessions(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM sessions WHERE expires_at <= NOW()")
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
