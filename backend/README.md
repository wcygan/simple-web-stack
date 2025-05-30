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