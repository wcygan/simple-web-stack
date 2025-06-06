# Production build (default)
FROM rust:latest AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY . .
RUN cargo build --bin backend

FROM debian:bookworm-slim AS runtime
WORKDIR /usr/local/bin
COPY --from=builder /app/target/debug/backend .
EXPOSE 3000
CMD ["./backend"]

# Development stage with hot reloading
FROM rust:latest AS development
WORKDIR /app

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

# Copy dependency files first for caching
COPY Cargo.toml Cargo.lock ./

# Pre-build dependencies (empty main.rs to satisfy cargo)
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && cargo build

# Remove the dummy main.rs
RUN rm -rf src

# Copy source code
COPY src ./src

# Expose port
EXPOSE 3000

# Use cargo-watch for hot reloading
CMD ["cargo", "watch", "-x", "run --bin backend"]