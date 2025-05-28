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
| `deno task test:backend` | Runs all backend tests (unit tests + integration tests). |

### Accessing Services:

*   **Frontend**: Accessible at [http://localhost:8000](http://localhost:8000) after running `deno task up`.
*   **Backend**: Accessible at [http://localhost:3000](http://localhost:3000) after running `deno task up`.

## Development

(Further development instructions to be added here, e.g., how to develop the Fresh frontend and the Rust backend, including live reloading, database migrations, etc.)

## Testing

### Backend Testing
The backend includes comprehensive testing at multiple levels:

- **Unit Tests**: Fast, isolated tests of individual components (embedded in Rust code)
- **Integration Tests**: End-to-end testing of the complete HTTP server (Deno script)

Run all backend tests with:
```bash
deno task test:backend
```

For more details, see the [backend README](backend/README.md).

