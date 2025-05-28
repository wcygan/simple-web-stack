# Simple Web Stack

| Component | Technology/Libraries |
|---|---|
| Frontend | [Deno Fresh](https://fresh.deno.dev/) |
| Backend | [Axum](https://docs.rs/axum/latest/axum/) |
| Database | [MySQL](https://www.mysql.com/) + [sqlx](https://github.com/launchbadge/sqlx) |
| Cache | [DragonflyDB](https://www.dragonflydb.io/) + [redis-rs](https://github.com/redis-rs/redis-rs) |

## Quick Start

This project uses Docker Compose to manage its services. The Deno tasks provide convenient wrappers for `docker compose` commands.

### Deno Tasks:

| Task | Description |
|---|---|
| `deno task up` | Builds and starts all services in detached mode (`docker compose up -d --build`). |
| `deno task down` | Stops and removes all services (`docker compose down`). |

### Accessing Services:

*   **Frontend**: Accessible at [http://localhost:8000](http://localhost:8000) after running `deno task up`.
*   **Backend**: Currently, the backend container runs a simple Rust program that prints "Hello, world!" to the container logs and then exits. It does not start an HTTP server. You can view its output by checking the Docker logs: `docker compose logs backend`.

## Development

(Further development instructions to be added here, e.g., how to develop the Fresh frontend and the Rust backend, including live reloading, database migrations, etc.)

