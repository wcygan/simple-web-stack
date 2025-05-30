# Hot Reloading

Setting up hot-reloading for both frontend and backend in a Dockerized environment is a common requirement for a smooth development experience. You're correct that the Deno Fresh frontend already supports this well with `deno task dev` (which uses file watching) when source code is volume-mounted into the Docker container.

For the Rust backend, the typical way to achieve hot-reloading without recompiling the Docker *image* for every change involves:
1.  **Volume Mounting Source Code**: Mount your Rust project's source code (`./backend/src`, `Cargo.toml`, etc.) into the backend container.
2.  **Using a Watcher Tool**: Inside the container, run a tool like `cargo-watch`. This tool monitors your Rust source files for changes and, upon detecting a change, automatically recompiles and restarts your Rust application.
3.  **Development-Specific Docker Configuration**: Use a development-focused Docker stage and `docker-compose.override.yml` file to enable hot-reloading behavior only during development, while keeping the base configuration production-ready.

## Docker Compose Strategy: Production Base + Development Overrides

The most idiomatic way to achieve different commands and configurations for production and development is using:

1.  **`docker-compose.yml` (Base/Production-focused):**
    *   Define services with configurations suitable for production or as a common base.
    *   Commands here should be optimized for production (e.g., running compiled binaries).
    *   This file is committed to your repository.

2.  **`docker-compose.override.yml` (Development Overrides):**
    *   Overrides or extends configurations from the base `docker-compose.yml`.
    *   Contains development-specific commands (like `deno task dev` or `cargo watch`).
    *   Adds development-specific volumes, environment variables, etc.
    *   Often *not* committed to the repository (can be added to `.gitignore`) or is specific to local development setups.

When you run `docker-compose up`, Docker Compose automatically reads `docker-compose.yml` first, then merges `docker-compose.override.yml` on top. Values in the override file take precedence.

## Implementation

### 1. Modify `backend/Dockerfile`

Create a multi-stage Dockerfile with distinct stages for development and production:

```dockerfile
# backend/Dockerfile

# Stage 1: Install cargo-chef and cargo-watch
FROM rust:latest AS chef
WORKDIR /app
RUN cargo install cargo-chef cargo-watch # <-- Added cargo-watch

# Stage 2: Create a recipe for dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies (Base for Dev and Prod builds)
FROM chef AS builder_base
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the Docker layer cached by cargo-chef
RUN cargo chef cook --recipe-path recipe.json

# Stage 3.1: Development Stage
# This stage inherits pre-built dependencies from builder_base.
# It will have the source code mounted and run cargo-watch.
FROM builder_base AS development
WORKDIR /app # Ensures commands run in the project root
# Copy source code - this provides a fallback if volumes aren't mounted,
# but will be overlaid by volume mounts in docker-compose.
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
# Default command for this stage (can be overridden in docker-compose.yml)
# Waits for file changes, then runs the 'backend' binary.
# Passes '--app-bind-address 0.0.0.0:3000' to your application.
CMD ["cargo", "watch", "-C", ".", "-q", "-x", "run --bin backend -- --app-bind-address 0.0.0.0:3000"]

# Stage 4: Final Build for Production (inherits from builder_base)
FROM builder_base AS builder
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
# Build the optimized release binary for production
RUN cargo build --bin backend --release

# Stage 5: Minimal Runtime Image for Production
FROM debian:bookworm-slim AS runtime
WORKDIR /usr/local/bin
# Copy the compiled release binary from the builder stage
COPY --from=builder /app/target/release/backend .
EXPOSE 3000
# Default command for production: runs the compiled binary
CMD ["./backend"]
```

### 2. Create Production-Focused `docker-compose.yml`

The base configuration should define how services run in production:

```yaml
# docker-compose.yml
version: '3.8'

services:
  frontend:
    container_name: simple_web_stack_frontend
    build:
      context: ./frontend
      dockerfile: Dockerfile
    ports:
      - "8000:8000"
    # Production command for frontend
    command: ["deno", "task", "start"] # Assumes 'deno task start' runs optimized for production

  mysql:
    image: mysql:8.0
    container_name: simple_web_stack_mysql
    ports:
      - "3306:3306"
    environment:
      MYSQL_ROOT_PASSWORD: rootpassword
      MYSQL_DATABASE: tasks_db
      MYSQL_USER: taskuser
      MYSQL_PASSWORD: taskpassword
    volumes:
      - mysql_data:/var/lib/mysql
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost", "-u$${MYSQL_USER}", "-p$${MYSQL_PASSWORD}"]
      interval: 10s
      timeout: 5s
      retries: 5

  backend:
    container_name: simple_web_stack_backend
    build:
      context: ./backend
      dockerfile: Dockerfile
      # Implicitly targets the last stage ('runtime') for production
      # Dockerfile's CMD for 'runtime' stage is ["./backend"] - the production command
      # No explicit 'command:' here means it uses the Dockerfile's CMD
    ports:
      - "3000:3000"
    environment:
      RUST_LOG: info # Production logging level
      DATABASE_URL: mysql://taskuser:taskpassword@mysql:3306/tasks_db
      APP_BIND_ADDRESS: 0.0.0.0:3000
    depends_on:
      mysql:
        condition: service_healthy

volumes:
  mysql_data:
```

### 3. Create `docker-compose.override.yml`

This file will modify your base `docker-compose.yml` specifically for development. Docker Compose automatically picks up `docker-compose.override.yml` if it exists in the same directory.

```yaml
# docker-compose.override.yml
version: '3.8'

services:
  frontend:
    # Development command for frontend with hot-reloading
    command: ["deno", "task", "dev"]
    volumes: # Ensure volumes are mounted for dev server to see host changes
      - ./frontend:/app

  backend:
    # Override the backend service for development
    build:
      context: ./backend
      dockerfile: Dockerfile
      target: development # Explicitly target the 'development' stage from backend/Dockerfile
    volumes:
      # Mount the backend source code. Changes on host reflect in container.
      - ./backend:/app
      # Mount a named volume for the target directory.
      # This persists Rust build artifacts (like .rlib, .o files) across container runs,
      # speeding up recompilations by cargo-watch.
      # It also keeps the host's ./backend/target directory clean if it exists.
      - backend_target_cache:/app/target
    # The command is inherited from the 'development' stage's CMD in Dockerfile:
    # ["cargo", "watch", "-C", ".", "-q", "-x", "run --bin backend -- --app-bind-address 0.0.0.0:3000"]
    # You can explicitly override it here if needed:
    # command: ["cargo", "watch", "-C", "/app", "-q", "-x", "run --bin backend -- --app-bind-address 0.0.0.0:3000"]
    # Ensure env vars for development are suitable
    environment:
      RUST_LOG: info,backend=debug,tower_http=trace,sqlx=warn # More verbose logging for dev
      RUST_BACKTRACE: "1" # Enable backtraces for panics
      # DATABASE_URL and APP_BIND_ADDRESS are typically inherited from the base docker-compose.yml
      # but can be overridden here if needed for dev.
      # For cargo watch to pass CLI args to your app, ensure APP_BIND_ADDRESS
      # is either correctly set in ENV or passed through the cargo watch -x command.
      # The CMD in the Dockerfile's 'development' stage already handles this.

volumes:
  backend_target_cache: # Define the named volume used above
```

## How it Works

### Development Mode (with Override)

When you run `docker-compose up --build` (or `deno task up`):

1.  **Frontend Hot-Reloading**:
    *   Docker Compose merges `docker-compose.yml` and `docker-compose.override.yml`.
    *   The `frontend` service command is overridden to `["deno", "task", "dev"]`.
    *   The `./frontend` directory is mounted into `/app` in the container.
    *   Deno's development server with built-in file watcher (`--watch` flag in `dev.ts`) detects changes and restarts the Fresh server.

2.  **Backend Hot-Reloading**:
    *   The `backend` service targets the `development` stage of `backend/Dockerfile`.
    *   This stage has Rust, pre-built dependencies (cached by `cargo-chef`), and `cargo-watch`.
    *   `./backend` on your host is mounted to `/app` in the container.
    *   The `backend_target_cache` named volume preserves Rust build artifacts between runs.
    *   The `CMD` from the `development` stage runs `cargo watch ...`.
    *   When you change a `.rs` file in `./backend/src` on your host:
        *   The change is reflected inside the container at `/app/src`.
        *   `cargo-watch` detects this change.
        *   It re-runs `cargo run --bin backend ...`. Since dependencies are cached and Rust's incremental compilation is efficient, this is much faster than rebuilding the entire Docker image.
        *   Your Axum application restarts within the container.

### Production Mode (without Override)

For production-like builds/deployments:

*   When you deploy, you typically *don't* include the `docker-compose.override.yml` file.
*   Docker Compose will *only* use `docker-compose.yml`.
*   The `frontend` service will run `deno task start` (optimized production command).
*   The `backend` service will build the final `runtime` Docker image stage and run the compiled `./backend` binary.
*   No source code volumes are mounted, and no development tools are included.

## Usage

### For Development:
```bash
# Uses both docker-compose.yml and docker-compose.override.yml automatically
deno task up
```

### For Production-like Testing:
```bash
# Ignore the override file to test production configuration
docker-compose -f docker-compose.yml up --build
```

### Alternative Override File Naming:
Some developers prefer explicit naming for clarity:
```bash
# Using explicitly named development file
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

## Team-Friendly Configuration (Recommended)

For teams that want to share development configurations in git while maintaining flexibility, use explicit file naming instead of the automatic `docker-compose.override.yml` pickup:

### File Structure for Teams:
```
├── docker-compose.yml          # Production base (committed)
├── docker-compose.dev.yml      # Development overrides (committed) 
├── docker-compose.prod.yml     # Production overrides (committed)
└── docker-compose.local.yml    # Personal local overrides (gitignored)
```

### Updated `deno.json` Tasks:
```json
{
  "tasks": {
    "up": "docker-compose -f docker-compose.yml -f docker-compose.dev.yml up --build",
    "up:prod": "docker-compose -f docker-compose.yml -f docker-compose.prod.yml up --build",
    "down": "docker-compose -f docker-compose.yml -f docker-compose.dev.yml down",
    "logs": "docker-compose -f docker-compose.yml -f docker-compose.dev.yml logs -f"
  }
}
```

### Team Development File Example:
```yaml
# docker-compose.dev.yml - Shared development configuration
version: '3.8'

services:
  frontend:
    command: ["deno", "task", "dev"]
    volumes:
      - ./frontend:/app

  backend:
    build:
      target: development
    volumes:
      - ./backend:/app
      - backend_target_cache:/app/target
    environment:
      RUST_LOG: info,backend=debug,tower_http=trace,sqlx=warn
      RUST_BACKTRACE: "1"

volumes:
  backend_target_cache:
```

### Personal Customizations (Optional):
Create a `docker-compose.local.yml` file for personal development preferences:
```yaml
# docker-compose.local.yml - Personal overrides (not committed)
version: '3.8'

services:
  backend:
    environment:
      RUST_LOG: trace  # More verbose logging for debugging
    ports:
      - "3001:3000"    # Different port to avoid conflicts
```

Usage with personal overrides:
```bash
# Include personal customizations
docker-compose -f docker-compose.yml -f docker-compose.dev.yml -f docker-compose.local.yml up
```

### Benefits of Team-Friendly Structure:

*   **Shared Development Environment**: All team members get identical development setups
*   **Version Controlled**: Development configurations are tracked in git
*   **Personal Flexibility**: Individual developers can add personal overrides without affecting the team
*   **Explicit Configuration**: Clear visibility into which configurations are being applied
*   **Production Parity**: Easy testing of production-like configurations

According to the [Docker Compose documentation](https://docs.docker.com/compose/how-tos/multiple-compose-files/merge/), when using multiple `-f` flags, "files merge or override the configuration values from previous files, in the order they're specified on the command line."

## Key Benefits

*   **Clean Separation**: Production configuration in `docker-compose.yml` remains clean and deployment-ready.
*   **Development Flexibility**: Override file provides all development conveniences without affecting production.
*   **Automatic Merging**: Docker Compose handles the configuration merging automatically.
*   **Fast Development Loop**: Hot-reloading avoids full Docker image rebuilds for code changes.
*   **Cached Dependencies**: Rust build artifacts persist between container runs, speeding up recompilations.

## Best Practices

*   **Override File in .gitignore**: Consider adding `docker-compose.override.yml` to `.gitignore` if it contains developer-specific configurations.
*   **Build Target Specification**: Always explicitly specify `target: development` in override files to ensure the correct Docker stage is built.
*   **Volume Strategy**: Use named volumes for build artifacts (`backend_target_cache`) to persist between container runs while keeping host directories clean.
*   **Environment Separation**: Use different logging levels and debugging configurations between development and production.

## References:

1. [Docker Compose Production Guide](https://docs.docker.com/compose/how-tos/production/)
2. [Multiple Compose Files](https://docs.docker.com/compose/how-tos/multiple-compose-files/)
3. [Docker Compose Override Files](https://nickjanetakis.com/blog/a-docker-compose-override-file-can-help-avoid-compose-file-duplication)
4. [Using Multiple Docker Compose Files](https://medium.com/@alexkrenitsky/how-to-use-multiple-docker-compose-files-plus-profiles-69134c6a8d12)