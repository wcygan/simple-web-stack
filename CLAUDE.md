# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Simple Web Stack is a full-stack todo application with:
- **Frontend**: Deno Fresh 2.0 (Preact with Islands architecture)
- **Backend**: Rust/Axum with MySQL (via SQLx)
- **Infrastructure**: Docker Compose orchestration

## Essential Commands

### Development
```bash
# Start all services (frontend + backend + database)
deno task up

# Stop all services
deno task down

# Frontend development (hot reload at http://localhost:8000)
cd frontend && deno task dev

# Backend development (requires MySQL running)
cd backend && cargo run
```

### Testing
```bash
# Run backend tests optimally (parallel units, sequential integration)
deno task test:backend

# Run specific backend test
cd backend && cargo test test_name

# Run backend tests in watch mode
deno task test:backend:watch

# Run frontend tests
cd frontend && deno task test

# Run single frontend test
cd frontend && deno test --filter="test name"
```

### CI/Formatting/Linting
```bash
# Run backend CI locally
deno task ci:backend

# Run frontend CI locally  
deno task ci:frontend

# Format and lint frontend
deno task lint:frontend

# Backend formatting/linting
cd backend && cargo fmt && cargo clippy
```

## Architecture Notes

### API Structure
- Backend runs on port 3000, frontend on port 8000
- RESTful endpoints: `/tasks` (GET, POST), `/tasks/:id` (GET, PUT, DELETE)
- Frontend proxies API calls through `/api/tasks/*` routes

### State Management
- Frontend uses Preact Signals for reactive state
- TodoApp island manages the main application state
- API responses automatically update UI via signals

### Testing Strategy
Backend tests use a sophisticated hybrid approach:
- **Unit tests**: Run in parallel using all CPU cores
- **Integration tests**: Sequential by default, each gets isolated MySQL via Testcontainers
- **Performance tests**: Always sequential, resource-intensive
- Test script (`/scripts/test-backend.ts`) intelligently determines optimal execution strategy

### Database
- MySQL 8.0 with automatic schema initialization
- Connection pooling via SQLx
- Environment config: `DATABASE_URL` or CLI args
- Each integration test gets its own database for isolation

### Key Files
- `/backend/src/routes.rs` - API endpoint handlers
- `/frontend/islands/TodoApp.tsx` - Main application logic
- `/scripts/test-backend.ts` - Sophisticated test runner
- `/docker-compose.yml` - Service orchestration