# Stage 1: Install cargo-chef and prepare the build recipe
FROM rust:latest AS chef
WORKDIR /app
RUN cargo install cargo-chef

# Stage 2: Create a recipe for dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

# Copy application code and build the application
COPY . .
# Ensure the target directory from the cook stage is available for the final build
# Build the application binary
RUN cargo build --bin backend

# Stage 4: Create a minimal runtime image
FROM debian:bookworm-slim AS runtime
WORKDIR /usr/local/bin
# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/debug/backend .
EXPOSE 3000
CMD ["./backend"]