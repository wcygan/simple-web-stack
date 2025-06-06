# Project Plan: Add Pagination to Simple Web Stack

## Overview
This plan outlines the implementation of pagination for the Simple Web Stack todo application. The project currently fetches all tasks at once, which will not scale well. We'll add server-side pagination to both the backend API and frontend UI.

## Current State Assessment
- **Backend**: Rust/Axum API returns all tasks in a single response
- **Frontend**: Deno Fresh app displays all tasks without pagination controls
- **Database**: MySQL with tasks table, no pagination queries
- **Existing Plan**: Detailed PAGINATION.md already exists in /plans/

## Task Breakdown

### Phase 1: Parallel Tasks
Tasks that can be executed simultaneously by multiple agents

#### Task 1.1: Backend Core Pagination
- **Description**: Implement pagination infrastructure in Rust backend
- **Files**: 
  - `/backend/src/models.rs` - Add pagination structs
  - `/backend/src/routes.rs` - Modify list_tasks handler
- **Dependencies**: None (can run in parallel)
- **Estimated Time**: 2 hours
- **Agent Assignment**: Agent A
- **Details**:
  - Add `PaginatedResponse<T>`, `PaginationMeta`, `PaginationParams` structs
  - Implement pagination query logic with LIMIT/OFFSET
  - Add total count query
  - Update response format

#### Task 1.2: Frontend Type Definitions & API Client
- **Description**: Update frontend types and API client for pagination
- **Files**: 
  - `/frontend/types.ts` - Add pagination types
  - `/frontend/utils.ts` - Add pagination utilities
  - `/frontend/routes/api/tasks/index.tsx` - Forward query params
- **Dependencies**: None (can run in parallel)
- **Estimated Time**: 1.5 hours
- **Agent Assignment**: Agent B
- **Details**:
  - Mirror backend pagination types
  - Create helper functions for page calculations
  - Update API proxy to forward pagination params

#### Task 1.3: Database Optimization
- **Description**: Add database indexes for pagination performance
- **Files**: 
  - `/backend/src/db.rs` - Add migration for indexes
  - New file: `/backend/migrations/add_pagination_indexes.sql`
- **Dependencies**: None (can run in parallel)
- **Estimated Time**: 1 hour
- **Agent Assignment**: Agent C
- **Details**:
  - Create index on `created_at DESC`
  - Add composite indexes if needed
  - Test query performance

### Phase 2: Join Point 1
- **Duration**: 30 minutes
- **Actions**:
  - Merge parallel work from Phase 1
  - Run backend tests to ensure API compatibility
  - Verify type definitions match between frontend/backend
  - Resolve any merge conflicts

### Phase 3: Sequential Frontend Implementation

#### Task 3.1: Update TodoApp State Management
- **Description**: Refactor TodoApp to handle paginated data
- **Files**: `/frontend/islands/TodoApp.tsx`
- **Dependencies**: Phase 2 completion
- **Estimated Time**: 2 hours
- **Details**:
  - Replace single tasks array with paginated state
  - Add signals: currentPage, totalPages, itemsPerPage
  - Update fetchTasks to fetchPaginatedTasks
  - Modify CRUD operations to handle pagination

#### Task 3.2: Create Pagination Controls Component
- **Description**: Build reusable pagination UI component
- **Files**: 
  - New file: `/frontend/islands/PaginationControls.tsx`
  - New file: `/frontend/components/PaginationControls.tsx`
- **Dependencies**: Task 3.1
- **Estimated Time**: 2 hours
- **Details**:
  - Previous/Next buttons with disabled states
  - Page number display
  - Optional: Jump to page, items per page selector
  - Responsive design with Tailwind

#### Task 3.3: Integrate Pagination UI
- **Description**: Add pagination controls to main UI
- **Files**: 
  - `/frontend/islands/TodoApp.tsx`
  - `/frontend/static/styles.css` (if needed)
- **Dependencies**: Task 3.2
- **Estimated Time**: 1 hour
- **Details**:
  - Position controls appropriately
  - Connect controls to state
  - Handle edge cases (empty results, single page)

### Phase 4: Testing & Polish

#### Task 4.1: Backend Integration Tests
- **Description**: Add comprehensive pagination tests
- **Files**: 
  - New file: `/backend/tests/tasks_pagination.rs`
  - Update: `/backend/tests/common/mod.rs`
- **Dependencies**: Task 3.3
- **Estimated Time**: 2 hours
- **Details**:
  - Test various page sizes
  - Test edge cases (empty pages, out of bounds)
  - Test sorting with pagination
  - Performance tests with large datasets

#### Task 4.2: Frontend Component Tests
- **Description**: Test pagination UI and logic
- **Files**: 
  - New file: `/frontend/tests/pagination_test.ts`
  - Update: `/frontend/tests/components_test.ts`
- **Dependencies**: Task 3.3
- **Estimated Time**: 1.5 hours
- **Details**:
  - Test PaginationControls component
  - Test TodoApp pagination behavior
  - Test URL sync (if implemented)

### Phase 5: Optional Enhancements

#### Task 5.1: URL State Synchronization
- **Description**: Sync pagination state with URL for bookmarking
- **Files**: 
  - `/frontend/islands/TodoApp.tsx`
  - `/frontend/routes/index.tsx`
- **Dependencies**: Phase 4 completion
- **Estimated Time**: 1 hour
- **Details**:
  - Add ?page=X to URL
  - Handle browser back/forward
  - Restore state from URL on load

#### Task 5.2: Advanced Features
- **Description**: Add items-per-page selector and search
- **Files**: Various frontend files
- **Dependencies**: Task 5.1
- **Estimated Time**: 2 hours
- **Details**:
  - Dropdown for 10/25/50 items per page
  - Search with pagination reset
  - Keyboard shortcuts for navigation

## Execution Strategy

### Multi-Agent Setup
```bash
# Terminal 1 - Agent A (Backend)
git checkout -b feature/pagination-backend
cd backend
# Work on Task 1.1

# Terminal 2 - Agent B (Frontend Types)
git checkout -b feature/pagination-frontend-types
cd frontend
# Work on Task 1.2

# Terminal 3 - Agent C (Database)
git checkout -b feature/pagination-db
cd backend
# Work on Task 1.3
```

### Coordination Points
1. **Phase 2 Join**: All agents push branches, one agent creates integration branch
2. **Testing Phase**: Coordinate test data and scenarios
3. **Final Integration**: Code review and merge to main

### Communication
- Use PR comments for async communication
- Update this PLAN.md with completion status
- Flag blockers immediately in shared Slack/Discord

## Success Criteria
- [ ] Backend returns paginated responses with meta information
- [ ] Frontend displays pagination controls
- [ ] Navigation between pages works smoothly
- [ ] CRUD operations handle pagination correctly
- [ ] All existing tests pass
- [ ] New pagination tests pass
- [ ] Performance: <100ms response for pages with 10-50 items
- [ ] UI is responsive and accessible
- [ ] Documentation updated

## Risk Mitigation
- **Risk**: Breaking API changes affect existing clients
  - **Mitigation**: Keep non-paginated endpoint temporarily, add versioning
- **Risk**: Performance degradation with COUNT queries
  - **Mitigation**: Add caching, consider approximate counts
- **Risk**: Complex state management in frontend
  - **Mitigation**: Incremental implementation, thorough testing

## Progress Tracking
- [ ] Phase 1: Backend infrastructure (0/3 tasks)
- [ ] Phase 2: Integration checkpoint
- [ ] Phase 3: Frontend implementation (0/3 tasks)
- [ ] Phase 4: Testing (0/2 tasks)
- [ ] Phase 5: Enhancements (0/2 tasks)

## References
- Existing plan: `/plans/PAGINATION.md`
- Backend routes: `/backend/src/routes.rs`
- Frontend app: `/frontend/islands/TodoApp.tsx`
- API docs: `/backend/README.md`