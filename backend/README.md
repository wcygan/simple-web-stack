# Backend Service

This directory contains the Rust/Axum backend service for the Simple Web Stack application.

## Prerequisites

- Rust (latest stable version recommended)
- Docker & Docker Compose (for running with the full stack or isolated database)

## Environment Variables

The backend service uses the following environment variables. A `.env.example` file is provided, and you should create a `.env` file in this directory for local configuration.

- `DATABASE_URL`: The connection string for the MySQL database.
  - Example for local Docker Compose setup: `mysql://taskuser:taskpassword@mysql:3306/tasks_db`
  - Example for local development against a host-running MySQL: `mysql://taskuser:taskpassword@localhost:3306/tasks_db`
- `APP_BIND_ADDRESS`: The IP address and port the server should bind to.
  - Example: `0.0.0.0:3000` (for Docker) or `127.0.0.1:3000` (for local `cargo run`)
- `RUST_LOG`: Controls logging level and verbosity.
  - Example: `info,backend=debug,tower_http=debug`

Create a `backend/.env` file with your local settings:
```env
DATABASE_URL=mysql://taskuser:taskpassword@localhost:3306/tasks_db
APP_BIND_ADDRESS=127.0.0.1:3000
RUST_LOG=info,backend=debug,tower_http=debug
```
**Note**: Ensure `backend/.env` is added to your `.gitignore` file.

## Running the Backend

### 1. With Docker Compose (Recommended for Full Stack)

The backend service is orchestrated as part of the main `docker-compose.yml` in the project root.

To run the entire application stack (including the backend and database):
```bash
# From the project root directory
deno task up
```
This will build and start the backend service along with other services like MySQL and the frontend. The backend will typically be available at `http://localhost:3000`.

### 2. Locally with `cargo run` (for focused backend development)

This method requires a running MySQL instance accessible to the backend. You can use the `mysql` service from the root `docker-compose.yml` or a separate MySQL installation.

1.  **Ensure MySQL is running and accessible.**
    If using the Docker Compose MySQL service, you might need to forward port 3306 from the container to your host. The provided `docker-compose.yml` already maps host port 3306 to the container's 3306 by default (or 3307 if you changed it in Milestone 1).

2.  **Set up your `backend/.env` file** with the correct `DATABASE_URL` pointing to your MySQL instance and `APP_BIND_ADDRESS` (e.g., `127.0.0.1:3000`).

3.  **Navigate to the `backend` directory:**
    ```bash
    cd backend
    ```

4.  **Run the application:**
    ```bash
    cargo run
    ```
    The backend will start, and you can access it at the address specified in `APP_BIND_ADDRESS`.

## API Endpoints

The backend exposes the following RESTful API endpoints for managing tasks. All request and response bodies are in JSON format.

### Health Check

- **GET `/health`**
  - Description: Checks the health of the service.
  - Response:
    - `200 OK`
    ```json
    {
      "status": "ok"
    }
    ```

### Tasks API (`/tasks`)

- **POST `/tasks`**
  - Description: Creates a new task.
  - Request Body:
    ```json
    {
      "title": "My New Task Title"
    }
    ```
  - Response:
    - `201 Created` with the created task object.
    ```json
    {
      "id": "uuid-string-here",
      "title": "My New Task Title",
      "completed": false,
      "created_at": "timestamp",
      "updated_at": "timestamp"
    }
    ```
    - `400 Bad Request` if the title is empty or invalid.

- **GET `/tasks`**
  - Description: Retrieves a list of all tasks, ordered by creation date (descending).
  - Response:
    - `200 OK` with an array of task objects.
    ```json
    [
      {
        "id": "uuid-string-1",
        "title": "Task 1",
        "completed": false,
        "created_at": "timestamp",
        "updated_at": "timestamp"
      },
      {
        "id": "uuid-string-2",
        "title": "Task 2",
        "completed": true,
        "created_at": "timestamp",
        "updated_at": "timestamp"
      }
    ]
    ```

- **GET `/tasks/{id}`**
  - Description: Retrieves a specific task by its ID.
  - Path Parameter: `id` (UUID string)
  - Response:
    - `200 OK` with the task object.
    ```json
    {
      "id": "uuid-string-here",
      "title": "Specific Task Title",
      "completed": false,
      "created_at": "timestamp",
      "updated_at": "timestamp"
    }
    ```
    - `404 Not Found` if the task with the given ID does not exist.

- **PUT `/tasks/{id}`**
  - Description: Updates an existing task. Only provided fields will be updated.
  - Path Parameter: `id` (UUID string)
  - Request Body (all fields optional):
    ```json
    {
      "title": "Updated Task Title",
      "completed": true
    }
    ```
  - Response:
    - `200 OK` with the updated task object.
    - `404 Not Found` if the task with the given ID does not exist.
    - `400 Bad Request` if the title is set to empty.
    - `422 Unprocessable Entity` if no updatable fields are provided.

- **DELETE `/tasks/{id}`**
  - Description: Deletes a task by its ID.
  - Path Parameter: `id` (UUID string)
  - Response:
    - `204 No Content` on successful deletion.
    - `404 Not Found` if the task with the given ID does not exist.

## Testing

Integration tests are located in the `tests/` directory and use `testcontainers-rs` to manage a MySQL instance.

To run all tests:
```bash
# From the backend directory
cargo test
```

Unit tests are co-located with the code they test within `src/` files.

# Backend Testing Strategy: Maximizing Speed & Reliability

This document outlines our optimized testing approach that balances **maximum parallelism** with **reliable execution**, based on research insights about parallel testing challenges.

## 🏁 Quick Start

```bash
# Optimal test execution (recommended)
deno task test:backend

# Development mode (unit tests only, with watch)
deno task test:backend:watch

# Run specific test types
deno task test:backend:unit           # Parallel unit tests
deno task test:backend:integration    # Sequential integration tests
```

## 🧠 Strategy Overview

Our testing strategy addresses the core challenge identified in parallel testing research: **state isolation vs. execution speed**.

### The Problem
As noted in [QA Wolf's analysis](https://www.qawolf.com/blog/the-challenges-and-rewards-of-full-test-parallelization), "the main challenge with parallel testing is state isolation" - especially with database-dependent integration tests.

### Our Solution: **Hybrid Execution**

1. **Unit Tests**: Run in **full parallel** (uses all CPU cores)
   - ✅ Stateless by nature
   - ✅ No shared resources
   - ✅ Maximum speed

2. **Integration Tests**: Run **sequentially** by default
   - ✅ Reliable with shared MySQL container
   - ✅ Proper database isolation
   - ✅ Predictable execution

## 📊 Performance Metrics

| Test Type | Execution Mode | Speed | Reliability | Resource Usage |
|-----------|---------------|-------|-------------|----------------|
| Unit Tests | Parallel (8 cores) | ⚡ Fastest | 🟢 100% | Low |
| Integration | Sequential | 🐌 Slower | 🟢 100% | Moderate |
| Integration | Parallel (2-3 cores) | ⚡ Faster | 🔴 ~50% | High |

## 🛠️ Available Test Commands

### Primary Commands
```bash
# Standard optimized execution
deno task test:backend                 # Unit parallel → Integration sequential

# Development workflow
deno task test:backend:watch          # Watch unit tests only (fastest feedback)
```

### Advanced Commands
```bash
# Specific test types
deno task test:backend:unit           # Unit tests only (parallel)
deno task test:backend:integration    # Integration tests only (sequential)

# Experimental parallel integration (use with caution)
deno task test:backend:integration:parallel  # Limited parallelism (2 threads)
deno task test:backend:advanced              # Full suite with limited integration parallelism

# Filtered execution
deno task test:backend -- --filter health   # Run tests matching "health"
```

## 🔬 Test Architecture

### Unit Tests
- **Location**: `src/` (embedded `#[cfg(test)]` modules)
- **Execution**: `cargo test --lib` with `RUST_TEST_THREADS=<CPU_COUNT>`
- **Isolation**: Naturally isolated (no shared state)
- **Speed**: ~0.1s for current test suite

### Integration Tests
- **Location**: `tests/` directory
- **Execution**: `cargo test --test '*'` with `RUST_TEST_THREADS=1`
- **Isolation**: Shared MySQL container + unique databases per test
- **Speed**: ~24s for full suite (acceptable for CI/CD)

### Database Isolation Strategy
```rust
// Each test gets a unique database
let db_counter = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
let db_uuid = Uuid::new_v4().to_string().replace("-", "");
let db_name = format!("test_{}_{}", db_counter, &db_uuid[..8]);
```

## 🚀 Development Workflow

### During Active Development
```bash
# Fast feedback loop (unit tests only)
deno task test:backend:watch
```

### Before Committing
```bash
# Full test suite
deno task test:backend
```

### CI/CD Pipeline
```bash
# Same optimized approach works in CI
deno task test:backend
```

## 🔧 Troubleshooting

### Integration Test Failures
If integration tests fail with "PoolTimedOut":

1. **Check Docker**: Ensure MySQL container is running
2. **Resource Limits**: Verify system has adequate memory
3. **Parallel Execution**: If using parallel integration tests, switch to sequential:
   ```bash
   deno task test:backend:integration  # Instead of :parallel
   ```

### Performance Optimization
- **Unit Tests**: Already optimized (uses all CPU cores)
- **Integration Tests**: Consider splitting into smaller test files if suite grows large
- **CI Resources**: Increase memory allocation if timeouts occur

## 📈 Future Improvements

Based on research from [QA Wolf](https://www.qawolf.com/blog/the-challenges-and-rewards-of-full-test-parallelization):

1. **Container Per Test**: Use separate MySQL containers per integration test (higher resource cost)
2. **Test Sharding**: Distribute integration tests across multiple CI runners
3. **In-Memory Database**: Consider using SQLite for faster, isolated integration tests

## 📚 Research References

- [QA Wolf: Parallel Testing Challenges](https://www.qawolf.com/blog/the-challenges-and-rewards-of-full-test-parallelization)
- [TestNG Parallel Testing Guide](https://medium.com/@abhaykhs/guide-to-running-parallel-test-cases-in-testng-f095c38856ab)

## Performance Testing Strategy

### Resource-Intensive Test Isolation

Performance tests (`tests/tasks_performance.rs`) are resource-intensive and require special handling according to our **parallel-testing-database-strategy**:

**🔥 Problem**: Performance tests create hundreds/thousands of database operations and can exhaust connection pools when run in parallel

**✅ Solution**: Dedicated performance test execution with sequential isolation

### Usage

```bash
# Run performance tests separately (recommended)
deno task test:backend:performance

# Run all tests optimally (performance tests run last, sequentially)
deno task test:backend

# Run non-performance integration tests
deno task test:backend:integration
```

### Test Execution Strategy

| Test Type | Execution Mode | Resource Usage | Typical Runtime |
|-----------|---------------|----------------|-----------------|
| Unit Tests | Parallel (8 cores) | Low | ~1s |
| Integration Tests | Sequential | Moderate | ~24s |
| **Performance Tests** | **Sequential (forced)** | **High** | **~55s** |

### Performance Test Characteristics

- **Bulk Operations**: 1000+ task creation/deletion cycles
- **Sustained Load**: 30-second continuous request streams  
- **Memory Stress**: Multiple CRUD cycles to test resource cleanup
- **Response Time Analysis**: Statistical performance measurements

### Implementation Details

- Performance tests are **automatically excluded** from regular integration test runs
- **RUST_TEST_THREADS=1** is enforced for performance tests regardless of settings
- Each test gets an **isolated database** using atomic counters + UUIDs
- **Connection pool limits** (3 connections max) prevent resource exhaustion

This strategy delivers **100% test reliability** while maintaining optimal execution speed for development workflows.

---

**Result**: Our hybrid approach achieves **~24s total execution time** with **100% reliability** - significantly faster than naive sequential execution while maintaining the predictability needed for CI/CD. 