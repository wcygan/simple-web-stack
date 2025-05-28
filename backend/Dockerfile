# Stage 1: Build the Rust application
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build dependencies separately to leverage Docker cache
RUN mkdir src \
    && echo "fn main() {println!(\"Dummy main for dep build\")}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Clean previous build artifacts before copying new source
RUN rm -rf target

# Copy the rest of the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim AS runtime

WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend .

# Expose the port the backend will listen on (assuming 3000)
EXPOSE 3000

# Run the backend application
CMD ["./backend"] 