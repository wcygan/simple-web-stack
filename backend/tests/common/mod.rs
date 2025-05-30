use axum::Router;
use once_cell::sync::Lazy;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Mutex,
};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::mysql::Mysql;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

// Import backend modules using the crate name directly
use backend::db::{init_db, AppState};
use backend::routes;

// Shared container state
static CONTAINER: Lazy<Mutex<Option<ContainerState>>> = Lazy::new(|| Mutex::new(None));

// Database allocation counter for parallel database isolation
static DB_COUNTER: AtomicU32 = AtomicU32::new(0);

struct ContainerState {
    _container: ContainerAsync<Mysql>,
    port: u16,
}

// Test configuration holder
pub struct TestApp {
    pub address: String,
    pub db_pool: MySqlPool,
    pub db_name: String,
}

// Build the application router - extracted for test reuse
pub fn build_app(app_state: AppState) -> Router {
    Router::new()
        .route("/health", axum::routing::get(routes::health_check))
        .nest("/tasks", routes::task_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

// Initialize the shared container
async fn ensure_container() -> u16 {
    // First, check if container already exists (quick check with dropped guard)
    {
        let container_guard = CONTAINER.lock().unwrap();
        if let Some(ref state) = *container_guard {
            return state.port;
        }
        // Guard is dropped here when scope ends
    }

    // Start MySQL container only once (without holding the lock)
    let container = Mysql::default()
        .with_env_var("MYSQL_ROOT_PASSWORD", "root")
        .with_env_var("MYSQL_DATABASE", "test_db")
        .with_env_var("MYSQL_USER", "test_user")
        .with_env_var("MYSQL_PASSWORD", "test_pass")
        .start()
        .await
        .expect("Failed to start MySQL container");

    let port = container
        .get_host_port_ipv4(3306)
        .await
        .expect("Failed to get MySQL port");

    // Wait for MySQL to be ready with retry logic
    let root_url = format!("mysql://root:root@127.0.0.1:{}/mysql", port);
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 10;

    loop {
        attempts += 1;

        match MySqlPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(10))
            .connect(&root_url)
            .await
        {
            Ok(pool) => {
                pool.close().await;
                break;
            }
            Err(e) if attempts < MAX_ATTEMPTS => {
                eprintln!(
                    "MySQL not ready yet (attempt {}/{}): {}",
                    attempts, MAX_ATTEMPTS, e
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
            Err(e) => {
                panic!(
                    "Failed to connect to MySQL after {} attempts: {}",
                    MAX_ATTEMPTS, e
                );
            }
        }
    }

    // Now acquire the lock again to store the result
    {
        let mut container_guard = CONTAINER.lock().unwrap();
        // Double-check that another thread didn't create it while we were working
        if container_guard.is_none() {
            *container_guard = Some(ContainerState {
                _container: container,
                port,
            });
        }
        // Guard is dropped here
    }

    port
}

// Spawn the application for testing
pub async fn spawn_app() -> TestApp {
    // Get or create the shared container
    let db_port = ensure_container().await;

    // Create unique database using atomic counter + UUID for better isolation
    let db_counter = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
    let db_uuid = Uuid::new_v4().to_string().replace("-", "");
    let db_name = format!("test_{}_{}", db_counter, &db_uuid[..8]);

    // Connect as root to create the test database
    let root_url = format!("mysql://root:root@127.0.0.1:{}/mysql", db_port);
    let root_pool = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&root_url)
        .await
        .expect("Failed to connect to MySQL");

    // Create test database
    sqlx::query(&format!("CREATE DATABASE {}", db_name))
        .execute(&root_pool)
        .await
        .expect("Failed to create test database");

    // Grant privileges to test_user on the new database
    sqlx::query(&format!(
        "GRANT ALL PRIVILEGES ON {}.* TO 'test_user'@'%'",
        db_name
    ))
    .execute(&root_pool)
    .await
    .expect("Failed to grant privileges");

    // Flush privileges to ensure they take effect
    sqlx::query("FLUSH PRIVILEGES")
        .execute(&root_pool)
        .await
        .expect("Failed to flush privileges");

    // Close root connection
    root_pool.close().await;

    // Connect to the test database with optimized pool settings
    let database_url = format!(
        "mysql://test_user:test_pass@127.0.0.1:{}/{}",
        db_port, db_name
    );
    let db_pool = MySqlPoolOptions::new()
        .max_connections(3) // Reduced for better resource management
        .acquire_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(std::time::Duration::from_secs(30)) // Cleanup idle connections
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Run database migrations
    init_db(&db_pool)
        .await
        .expect("Failed to run database migrations");

    // Bind to random port using tokio's TcpListener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // Create app state
    let app_state = AppState {
        pool: db_pool.clone(),
    };

    // Build the app
    let app = build_app(app_state);

    // Spawn the server
    let server = axum::serve(listener, app.into_make_service());

    tokio::spawn(async move {
        server.await.expect("Server failed");
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    TestApp {
        address,
        db_pool,
        db_name,
    }
}

// Cleanup helper
impl TestApp {
    pub async fn cleanup(&self) {
        // Get the container port from our shared state
        let db_port = {
            let container_guard = CONTAINER.lock().unwrap();
            container_guard.as_ref().map(|s| s.port).unwrap_or(3306)
        };

        // Drop the test database
        let root_url = format!("mysql://root:root@127.0.0.1:{}/mysql", db_port);

        if let Ok(root_pool) = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&root_url)
            .await
        {
            let _ = sqlx::query(&format!("DROP DATABASE IF EXISTS {}", self.db_name))
                .execute(&root_pool)
                .await;
            root_pool.close().await;
        }

        // Close the pool
        self.db_pool.close().await;
    }
}
