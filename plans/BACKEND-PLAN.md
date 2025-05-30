This plan outlines the steps to complete the backend implementation for the simple web stack, focusing on Rust with Axum, MySQL with SQLx, and Docker.

**Project Goals:**
*   A functional Rust/Axum backend service for CRUD operations on "Tasks".
*   MySQL database for persistence, managed via SQLx.
*   Dockerization of the backend service and MySQL.
*   `docker-compose.yml` to orchestrate the backend and database.
*   Comprehensive unit and integration tests (using Testcontainers-rs).
*   Adherence to the principles outlined in `.cursor/rules/backend-principles.mdc`.

**File Structure (Target):**
```
backend/
├── Cargo.toml
├── Dockerfile
├── .env.example      # Example environment variables
├── .env              # Local environment variables (in .gitignore)
├── src/
│   ├── main.rs       # Application entry point, Axum router setup, Clap config
│   ├── routes.rs     # Route handlers for tasks
│   ├── models.rs     # Task data structure (Serde, Sqlx derives)
│   ├── db.rs         # Database connection pool, schema initialization
│   └── errors.rs     # Custom error types and `IntoResponse` impl
└── tests/
    ├── common/
    │   └── mod.rs    # Shared test setup (Testcontainers, app spawning)
    ├── health_check.rs # Integration test for /health
    └── tasks_api.rs  # Integration tests for task CRUD operations
```

---

## Plan to Complete Backend Implementation

**Milestone 1: Core Backend Setup & Health Check**

1.  **Update `backend/Cargo.toml`:**
    *   Add necessary dependencies:
        ```toml
        [dependencies]
        axum = "0.7"
        tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
        tower = "0.4"
        tower-http = { version = "0.5", features = ["trace", "cors"] } # Added cors
        tracing = "0.1"
        tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
        serde = { version = "1", features = ["derive"] }
        serde_json = "1"
        sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "mysql", "uuid", "chrono", "macros"] }
        clap = { version = "4", features = ["derive", "env"] }
        uuid = { version = "1", features = ["v4", "serde"] }
        chrono = { version = "0.4", features = ["serde"] }
        dotenvy = "0.15" # For loading .env file

        [dev-dependencies]
        reqwest = { version = "0.12", features = ["json"] }
        testcontainers = "0.20" # Check latest version
        testcontainers-modules = { version = "0.8", features = ["mysql"] } # Check latest version for mysql module
        once_cell = "1" # For lazy static initialization in tests
        ```

2.  **Implement `backend/src/main.rs` (Initial Version):**
    *   Set up `tokio::main`.
    *   Initialize `tracing_subscriber` (read from `RUST_LOG` env var).
    *   Define a `Cli` struct with `clap::Parser` for configuration (e.g., listen address, `DATABASE_URL` with env fallback).
    *   Load `.env` file using `dotenvy::dotenv().ok();`.
    *   Create health check handler in `routes.rs` and mount it.
    *   Start Axum server based on `Cli` config.

3.  **Create `backend/src/routes.rs` (Initial Version):**
    *   Implement `health_check()` handler:
        ```rust
        use axum::{http::StatusCode, response::Json};
        use serde_json::{json, Value};

        pub async fn health_check() -> (StatusCode, Json<Value>) {
            (StatusCode::OK, Json(json!({"status": "ok"})))
        }
        ```

4.  **Configure `backend/Dockerfile`:**
    *   Verify it uses `cargo-chef` for efficient layer caching as specified in current `backend/Dockerfile`. This structure is good.
    *   Ensure it builds a release binary for production-like images (though `backend-principles.mdc` says no `--release` for early CI/local, final Docker image should ideally be optimized). For now, stick to debug build as per rule, but note this for future.

5.  **Update `docker-compose.yml`:**
    *   Add `mysql` service:
        ```yaml
        services:
          # ... frontend service ...
          mysql:
            image: mysql:8.0
            container_name: simple_web_stack_mysql
            ports:
              - "3306:3306" # Or map to a different host port like 3307:3306
            environment:
              MYSQL_ROOT_PASSWORD: rootpassword # Change in production
              MYSQL_DATABASE: tasks_db
              MYSQL_USER: taskuser
              MYSQL_PASSWORD: taskpassword # Change in production
            volumes:
              - mysql_data:/var/lib/mysql
            healthcheck: # Add healthcheck for MySQL
              test: ["CMD", "mysqladmin", "ping", "-h", "localhost", "-u$$MYSQL_USER", "-p$$MYSQL_PASSWORD"]
              interval: 10s
              timeout: 5s
              retries: 5

          backend:
            container_name: simple_web_stack_backend
            build:
              context: ./backend
              dockerfile: Dockerfile
            ports:
              - "3000:3000"
            environment:
              RUST_LOG: info,backend=debug,tower_http=debug # Example logging levels
              DATABASE_URL: mysql://taskuser:taskpassword@mysql:3306/tasks_db
              APP_BIND_ADDRESS: 0.0.0.0:3000 # Ensure backend binds to 0.0.0.0
            depends_on:
              mysql:
                condition: service_healthy # Wait for MySQL to be healthy
            # volumes: # For faster iteration during development if not relying on full rebuilds
            #   - ./backend/src:/app/src
            #   - ./backend/Cargo.toml:/app/Cargo.toml
            #   - ./backend/Cargo.lock:/app/Cargo.lock
        
        volumes:
          mysql_data: # Persist MySQL data
        ```

6.  **Create `backend/.env.example` and `backend/.env`:**
    *   `backend/.env.example`:
        ```
        DATABASE_URL=mysql://taskuser:taskpassword@localhost:3306/tasks_db
        RUST_LOG=info,backend=debug,tower_http=debug
        APP_BIND_ADDRESS=127.0.0.1:3000
        ```
    *   `backend/.env` (add this path to `backend/.gitignore`):
        ```
        DATABASE_URL=mysql://taskuser:taskpassword@localhost:3306/tasks_db # Matches docker-compose for local if port forwarded
        RUST_LOG=info,backend=debug,tower_http=debug
        APP_BIND_ADDRESS=127.0.0.1:3000
        ```
    *   Add `backend/.env` to the root `.gitignore`.

7.  **Test:** Run `docker-compose up --build`. Access `http://localhost:3000/health`.

---

**Milestone 2: Database Integration and Task Model**

1.  **Define `Task` Model (`backend/src/models.rs`):**
    ```rust
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    use uuid::Uuid;
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
    pub struct Task {
        pub id: Uuid,
        pub title: String,
        pub completed: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateTime<Utc>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateTime<Utc>>,
    }

    // Payload for creating a task (omits id, created_at, updated_at as they are auto-generated/managed)
    #[derive(Debug, Deserialize)]
    pub struct CreateTaskPayload {
        pub title: String,
    }

    // Payload for updating a task (all fields optional)
    #[derive(Debug, Deserialize)]
    pub struct UpdateTaskPayload {
        pub title: Option<String>,
        pub completed: Option<bool>,
    }
    ```

2.  **Implement Database Connection Module (`backend/src/db.rs`):**
    *   Create `AppState` struct (can be in `main.rs` or `db.rs`).
    *   Function `create_pool(database_url: &str) -> Result<sqlx::MySqlPool, sqlx::Error>`.
    *   Function `init_db(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error>` to execute schema creation:
        ```sql
        CREATE TABLE IF NOT EXISTS tasks (
            id CHAR(36) PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        );
        ```
    *   In `main.rs`, create the pool using `DATABASE_URL` from `Cli` config, run `init_db`, and add pool to Axum app state.

---

**Milestone 3: CRUD API Endpoints for Tasks**

1.  **Define Custom Error Handling (`backend/src/errors.rs`):**
    *   Create `AppError` enum (e.g., `SqlxError(sqlx::Error)`, `NotFound`, `ValidationError(String)`).
    *   Implement `axum::response::IntoResponse` for `AppError`.

2.  **Implement Task Handlers in `backend/src/routes.rs`:**
    *   `create_task(State<MySqlPool>, Json<CreateTaskPayload>) -> Result<(StatusCode, Json<Task>), AppError>`
    *   `get_task(State<MySqlPool>, Path<Uuid>) -> Result<Json<Task>, AppError>`
    *   `list_tasks(State<MySqlPool>) -> Result<Json<Vec<Task>>, AppError>`
    *   `update_task(State<MySqlPool>, Path<Uuid>, Json<UpdateTaskPayload>) -> Result<Json<Task>, AppError>`
    *   `delete_task(State<MySqlPool>, Path<Uuid>) -> Result<StatusCode, AppError>`
    *   Use `sqlx::query!`, `query_as!`, or `query_builder` for DB operations.
    *   Handle `sqlx::Error::RowNotFound` appropriately by returning `AppError::NotFound`.

3.  **Update Router in `backend/src/main.rs`:**
    *   Create `task_routes()` function in `routes.rs` that returns a `Router<AppState>`.
    *   Nest task routes under `/tasks`.
    *   Add `CorsLayer::permissive()` for frontend interaction.
    *   Add `TraceLayer::new_for_http()`.
    *   Pass the `AppState` (containing `MySqlPool`) to the router using `.with_state()`.

---

**Milestone 4: Testing**

### Key Ideas for Testcontainers Integration
* Single container reuse across tests to minimize startup overhead.
* Consistent database state through automated migrations and cleanup between tests.
* Isolated parallel test execution without interference.
* Easy setup and teardown via a shared TestContext abstraction.
* Realistic end-to-end testing against an actual MySQL container instance.

1.  **Set up `backend/tests/common/mod.rs`:**
    *   Function `spawn_app()` that:
        *   Spins up a MySQL container using `testcontainers` and `Mysql` module.
        *   Gets the dynamic connection string.
        *   Creates a `MySqlPool` for tests.
        *   Runs `init_db` (schema migration) on the test DB.
        *   Starts the Axum app on a random available port.
        *   Returns the app address (e.g., `http://127.0.0.1:XXXX`) and the test `MySqlPool`.

2.  **Write Integration Tests:**
    *   **`backend/tests/health_check.rs`:**
        *   Test `/health` endpoint using `reqwest`.
    *   **`backend/tests/tasks_api.rs`:**
        *   Use the `spawn_app()` helper.
        *   Test all CRUD endpoints:
            *   Successful creation, retrieval, listing, updating, deletion.
            *   Correct HTTP status codes and JSON response bodies.
            *   Error cases: task not found, invalid input (if validation is added later).

3.  **Write Unit Tests (as applicable):**
    *   For any helper functions or specific logic in `models.rs`, `db.rs`, or `errors.rs` that can be tested in isolation.

---

**Milestone 5: Refinements and Finalization**

1.  **Implement `clap` for CLI Configuration:**
    *   Finalize the `Cli` struct in `main.rs` for server port, bind address, and `DATABASE_URL` (with environment variable fallbacks).

2.  **Enhance Logging:**
    *   Ensure structured logging with relevant fields (e.g., `task_id`, `error_details`) is used throughout handlers and error pathways.

3.  **Code Quality:**
    *   Run `cargo fmt --all -- --check`.
    *   Run `cargo clippy --all -- -D warnings`. Address all lints.

4.  **Review and Verify:**
    *   Double-check all requirements from `.cursor/rules/backend-principles.mdc` are met.
    *   Test the full stack with `docker-compose up --build`.
    *   Manually test API endpoints using a tool like `curl` or Postman against the Dockerized service.

5.  **Documentation:**
    *   Update `README.md` in the root and/or create `backend/README.md` with:
        *   Instructions for running the backend (locally with `cargo run` and via Docker).
        *   Details on environment variables (`.env.example`).
        *   Brief API documentation (endpoints, request/response formats).

---

This plan provides a structured approach to completing the backend. Each milestone builds upon the previous one, ensuring a robust and testable application. The `backend-principles.mdc` file will serve as a constant reference.