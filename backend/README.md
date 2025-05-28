# Simple Web Stack Backend

A minimal Rust backend server built with Axum framework.

## Features

- **Health Check Endpoint**: `GET /health` - Returns server status and timestamp
- **Async Runtime**: Built on Tokio for high-performance async operations
- **JSON Responses**: All endpoints return JSON with proper content types
- **Docker Support**: Containerized with multi-stage build for minimal image size
- **Comprehensive Testing**: Unit tests in Rust and integration tests in Deno with timeout protection

## API Endpoints

### Health Check
```
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2025-05-28T04:37:39.827702310+00:00",
  "service": "simple-web-stack-backend"
}
```

## Development

### Prerequisites
- Rust 1.70+ 
- Cargo
- Deno (for integration tests)

### Running Locally
```bash
cargo run
```

The server will start on `http://localhost:3000`

### Building
```bash
cargo build --release
```

## Testing

This project includes comprehensive testing at multiple levels with timeout protection to prevent stalled tests:

### Unit Tests (Rust)
Unit tests are embedded in `src/main.rs` and test individual functions and handlers:

```bash
# Run unit tests directly
cargo test
```

**Unit test coverage includes:**
- Direct handler function testing (`health_check`)
- HTTP request/response integration via Axum's test utilities
- JSON response structure validation
- Timestamp format verification
- Error handling (404 responses)
- Multiple request handling
- Response schema validation

### Integration Tests (Deno)
Integration tests in `integration_test.ts` test the full HTTP server:

```bash
# Run integration tests directly
./integration_test.ts
```

**Integration test coverage includes:**
- Server startup and readiness
- HTTP status codes and headers
- JSON response structure and content
- Timestamp validity and recency
- Concurrent request handling
- Response time performance
- Error scenarios (invalid endpoints, malformed requests)
- Server cleanup and shutdown

### Timeout-Protected Test Runner
The recommended way to run tests is using the interactive timeout-protected test runner:

```bash
# Run all tests with timeout protection and real-time progress (recommended)
./test_runner.ts
```

**Interactive Features:**
- **Real-time Progress**: Animated spinners and elapsed time counters
- **Test Plan Overview**: Shows what tests will run and their timeouts
- **Live Status Updates**: Progress indicators for each test phase
- **Detailed Performance Metrics**: Breakdown of compilation and test execution times
- **Smart Recommendations**: Tips for optimization and troubleshooting

**Timeout Protection Features:**
- **Rust Unit Tests**: 5-second timeout to prevent stalled tests
- **Integration Tests**: 30-second timeout for complete server lifecycle testing
- **Automatic Cleanup**: Processes are properly terminated on timeout
- **Detailed Reporting**: Shows which tests timed out and why
- **Exit Codes**: Proper failure codes for CI/CD integration

**Example output:**
```
🚀 Starting backend test suite with timeout protection
============================================================
📋 Test Plan:
   1. 🦀 Rust unit tests (5s timeout)
   2. 🔗 Integration tests (30s timeout)
============================================================

📊 Progress:
   🦀 Rust unit tests: ⏳ Pending
   🔗 Integration tests: ⏳ Pending

✅ Rust unit tests passed (238ms)
   
📊 Progress Update:
   🦀 Rust unit tests: ✅ Completed
   🔗 Integration tests: ⏳ Starting...

✅ Integration tests passed (462ms)
   Completed 19 integration test scenarios

📊 Final Status:
   🦀 Rust unit tests: ✅ Completed
   🔗 Integration tests: ✅ Completed

============================================================
📊 DETAILED TEST SUMMARY
============================================================

⏱️  Performance Metrics:
   Total execution time: 700ms (0.7s)
   Rust compilation + tests: 238ms
   Integration test suite: 462ms

🦀 Rust Unit Tests:
   Status: ✅ PASSED
   Duration: 238ms

🔗 Integration Tests:
   Status: ✅ PASSED
   Duration: 462ms

🎯 Overall Result: ✅ ALL TESTS PASSED
🎉 Great job! All tests completed successfully in 0.7s
============================================================
```

### Test Architecture

**Unit Tests:**
- Use Axum's `tower::ServiceExt::oneshot` for testing handlers without starting a server
- Test individual components in isolation
- Fast execution (milliseconds)
- No external dependencies
- Protected by 5-second timeout

**Integration Tests:**
- Start actual Rust server as subprocess
- Test real HTTP requests against live server
- Validate full request/response cycle
- Test server lifecycle (startup/shutdown)
- Comprehensive error scenario testing
- Protected by 30-second timeout

**Timeout Protection:**
- Prevents CI/CD pipeline stalls
- Provides clear failure messages
- Maintains test reliability
- Enables safe automated testing

### Manual Testing
```bash
# Test the health endpoint
curl http://localhost:3000/health

# Test invalid endpoint (should return 404)
curl http://localhost:3000/invalid
```

## Docker

### Build Image
```bash
docker build -t simple-web-stack-backend .
```

### Run Container
```bash
docker run -p 3000:3000 simple-web-stack-backend
```

### Test Docker Container
```bash
# Test health endpoint in container
docker run -d -p 3000:3000 --name test-backend simple-web-stack-backend
curl http://localhost:3000/health
docker stop test-backend && docker rm test-backend
```

## Dependencies

### Runtime Dependencies
- **axum**: Web framework
- **tokio**: Async runtime
- **tower**: Service abstractions
- **serde**: Serialization framework
- **chrono**: Date and time handling

### Development Dependencies
- **tower** (with util features): Testing utilities
- **http-body-util**: HTTP body utilities for tests
- **mime**: MIME type handling

## Architecture

The server uses Axum's router-based architecture with:
- Async handlers for all endpoints
- JSON response serialization via serde
- Error handling with proper HTTP status codes
- Structured logging and timestamps
- Comprehensive test coverage at unit and integration levels

## Testing Philosophy

This project follows a comprehensive testing strategy:

1. **Unit Tests**: Fast, isolated tests of individual components
2. **Integration Tests**: End-to-end testing of the complete system
3. **Manual Testing**: Simple curl commands for quick verification
4. **Docker Testing**: Validation of containerized deployment

The combination ensures reliability at all levels from individual functions to the complete deployed system. 