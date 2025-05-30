use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::time::Duration;

/// Application state that will be shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
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

    // Create the tasks table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id CHAR(36) PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    tracing::info!("Database schema initialized successfully");
    Ok(())
}
