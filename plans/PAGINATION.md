# PAGINATION.MD

## 1. Overview and Goals

This document outlines the plan to implement pagination for the task list in the Simple Web Stack application. Pagination will be added to both the backend API and the frontend UI to improve performance and user experience when dealing with a large number of tasks.

**Goals:**
-   Allow users to navigate through tasks in pages.
-   Prevent loading all tasks at once, improving initial load time and reducing data transfer.
-   Implement a consistent pagination scheme across the backend and frontend.
-   Maintain a good user experience during task creation, deletion, and page navigation.

## 2. Backend Pagination Plan (Rust/Axum)

The backend will be updated to handle paginated requests for the task list, returning a subset of tasks along with metadata for pagination.

### 2.1. API Endpoint Changes (`GET /tasks`)

-   The `GET /tasks` endpoint will be modified to accept optional query parameters:
    -   `page`: The desired page number (1-indexed, defaults to 1).
    -   `limit`: The number of items per page (defaults to 10, with a reasonable maximum like 100).
-   Example: `GET /tasks?page=2&limit=5`

### 2.2. Database Query Modifications (`sqlx`)

-   **Fetch Paginated Tasks:**
    -   The SQL query to fetch tasks in `backend/src/routes.rs` (or a dedicated `db.rs` query function) will be updated to use `LIMIT` and `OFFSET` clauses.
    -   `OFFSET` will be calculated as `(page - 1) * limit`.
    -   Tasks will continue to be ordered (e.g., by `created_at DESC`).
    ```sql
    SELECT id, title, completed, created_at, updated_at
    FROM tasks
    ORDER BY created_at DESC
    LIMIT ? OFFSET ?;
    ```
-   **Fetch Total Task Count:**
    -   A separate SQL query will be executed to get the total number of tasks.
    ```sql
    SELECT COUNT(*) as total_items FROM tasks;
    ```

### 2.3. Response Structure

The `GET /tasks` endpoint will return a JSON object with the following structure:

```json
{
  "data": [
    {
      "id": "uuid-string",
      "title": "Task Title",
      "completed": false,
      "created_at": "timestamp",
      "updated_at": "timestamp"
    }
    // ... more tasks for the current page
  ],
  "meta": {
    "total_items": 100,    // Total number of tasks in the database
    "total_pages": 10,     // Total number of pages
    "current_page": 2,     // Current page number (1-indexed)
    "per_page": 10         // Items per page (limit used)
  }
}
```

### 2.4. Model and Handler Logic Changes

-   **`backend/src/models.rs`:**
    -   Define `PaginatedResponse<T>` struct to represent the new response structure.
    -   Define `PaginationMeta` struct for the metadata part.
      ```rust
      // In backend/src/models.rs
      use serde::Serialize; // Make sure to import Serialize

      #[derive(Debug, Serialize)]
      pub struct PaginatedResponse<T> {
          pub data: Vec<T>,
          pub meta: PaginationMeta,
      }

      #[derive(Debug, Serialize)]
      pub struct PaginationMeta {
          pub total_items: i64,
          pub total_pages: i64,
          pub current_page: i64,
          pub per_page: i64,
      }
      ```
-   **`backend/src/routes.rs` (or new `query_params.rs`):**
    -   Define a `PaginationParams` struct to deserialize `page` and `limit` query parameters with default values using `axum::extract::Query`.
      ```rust
      // In backend/src/routes.rs or a new module for request types
      use serde::Deserialize; // Make sure to import Deserialize

      const DEFAULT_PAGE: i64 = 1;
      const DEFAULT_LIMIT: i64 = 10;
      const MAX_LIMIT: i64 = 100;

      fn default_page_val() -> i64 { DEFAULT_PAGE }
      fn default_limit_val() -> i64 { DEFAULT_LIMIT }

      #[derive(Deserialize, Debug)]
      pub struct PaginationParams {
          #[serde(default = "default_page_val")]
          pub page: i64,
          #[serde(default = "default_limit_val")]
          pub limit: i64,
      }
      ```
-   **Handler for `list_tasks` in `backend/src/routes.rs`:**
    1.  Extract `page` and `limit` from `Query<PaginationParams>`.
    2.  Sanitize `page` (min 1) and `limit` (min 1, max `MAX_LIMIT`).
    3.  Calculate `offset = (page - 1) * limit`.
    4.  Execute the SQL query to fetch tasks with the sanitized `limit` and calculated `offset`.
    5.  Execute the SQL query to fetch the `total_items` count.
    6.  Calculate `total_pages = (total_items + limit - 1) / limit` (ensure integer division handles this correctly, or use `f64::ceil` and cast). If `total_items` is 0, `total_pages` should be 0 or 1 depending on preference (typically 0 if no items, 1 if items exist but fewer than `limit`).
    7.  Construct and return the `PaginatedResponse<Task>`.

### 2.5. Error Handling

-   The `Query<PaginationParams>` extractor in Axum will automatically handle basic parsing errors for `page` and `limit` (e.g., non-numeric values), typically returning a `400 Bad Request` or `422 Unprocessable Entity`.
-   Custom validation for `page > 0` and `0 < limit <= MAX_LIMIT` can be added in the handler, returning `AppError::ValidationError` if rules are violated.
-   Database errors during query execution should be mapped to `AppError::SqlxError` and result in a `500 Internal Server Error`.

### 2.6. Testing

-   Update integration tests in `backend/tests/tasks_api.rs`:
    -   Test `GET /tasks` with no query parameters (verify default `page=1`, `limit=10`).
    -   Test with valid `page` and `limit` parameters:
        -   Verify correct subset of tasks is returned, respecting `ORDER BY`.
        -   Verify the `meta` object contains correct `total_items`, `total_pages`, `current_page`, `per_page`.
        -   Ensure the number of tasks in `data` matches `limit` (or fewer, if on the last page).
    -   Test edge cases:
        -   `page` out of bounds (e.g., `page > total_pages`): Response should have an empty `data` array and correct `meta`.
        -   `page=0` or `page` negative: Should be handled by validation (e.g., default to 1 or return 400).
        -   `limit` exceeding `MAX_LIMIT`: Should be capped at `MAX_LIMIT` or return 400.
        -   `limit=0` or `limit` negative: Should be handled by validation (e.g., default to `DEFAULT_LIMIT` or return 400).
    -   Test with different numbers of tasks in the database (0, 1,
        `DEFAULT_LIMIT - 1`, `DEFAULT_LIMIT`, `DEFAULT_LIMIT + 1`, multiple pages worth).
    -   Ensure `created_at` and `updated_at` fields are present in the task objects within the `data` array.

## 3. Frontend Pagination Plan (Deno Fresh/Preact)

The frontend will be updated to request paginated data and provide UI controls for navigating pages.

### 3.1. API Interaction (`islands/TodoApp.tsx` or `frontend/utils/api.ts`)

-   Modify the `fetchTasks` function (or create a new `fetchPaginatedTasks`):
    -   It should accept `page` and `limit` as arguments.
    -   Append these as query parameters to the `GET /api/tasks` request (proxied via Fresh API routes).
    -   It should expect and correctly parse the new `PaginatedResponse` structure from the API.
    ```typescript
    // Located in islands/TodoApp.tsx or a separate frontend/utils/api.ts
    // API_BASE should point to Fresh's own /api proxy, e.g., "/api"

    interface ApiPaginationMeta { // Renamed to avoid conflict if Task type is also named Meta
      total_items: number;
      total_pages: number;
      current_page: number;
      per_page: number;
    }
    interface PaginatedTasksApiResponse {
      data: Task[]; // Assuming Task type is defined in frontend/types.ts
      meta: ApiPaginationMeta;
    }

    async function fetchPaginatedTasks(page: number, limit: number): Promise<PaginatedTasksApiResponse> {
      const response = await fetch(`${API_BASE}/tasks?page=${page}&limit=${limit}`);
      if (!response.ok) {
        const errorBody = await response.json().catch(() => ({ error: `HTTP error! Status: ${response.statusText}` }));
        throw new Error(errorBody.error || `Failed to fetch tasks: ${response.statusText}`);
      }
      return await response.json();
    }
    ```

### 3.2. State Management (`islands/TodoApp.tsx` using Preact Signals)

-   Update/Add signals in `TodoApp.tsx`:
    -   `tasks = useSignal<Task[]>([])` (will hold tasks for the *current* page).
    -   `currentPage = useSignal(1)`.
    -   `itemsPerPage = useSignal(10)` (can be a constant or a signal if user can change it).
    -   `totalPages = useSignal(0)`.
    -   `totalItems = useSignal(0)`.
    -   `isLoading = useSignal(true)` (re-use existing or refine for pagination).
    -   `error = useSignal<string | null>(null)` (re-use existing).
    -   `mounted = useSignal(false)` (if using the one-time load pattern).

-   Modify the `effect` hook in `TodoApp.tsx` that loads tasks:
    -   It should now depend on `currentPage.value` (and `itemsPerPage.value` if it's a signal).
    -   On mount and when dependencies change, call `fetchPaginatedTasks(currentPage.value, itemsPerPage.value)`.
    -   Update `tasks.value` with `result.data`.
    -   Update `totalPages.value` with `result.meta.total_pages`.
    -   Update `totalItems.value` with `result.meta.total_items`.
    -   Handle loading and error states appropriately.

    ```typescript
    // Inside TodoApp.tsx
    effect(() => {
      // const initialLoad = !mounted.value; // If special logic for initial load
      // if (!initialLoad && !currentPage.value) return; // Or similar guard

      const loadTasksForPage = async () => {
        isLoading.value = true;
        error.value = null;
        try {
          const result = await fetchPaginatedTasks(currentPage.value, itemsPerPage.value);
          tasks.value = result.data;
          totalPages.value = result.meta.total_pages;
          totalItems.value = result.meta.total_items;
        } catch (err) {
          console.error("Failed to load tasks:", err);
          error.value = (err as Error).message || "Failed to load tasks. Please try again.";
          tasks.value = []; // Clear tasks on error
          totalPages.value = 0;
          totalItems.value = 0;
        } finally {
          isLoading.value = false;
          // if (initialLoad) mounted.value = true;
        }
      };
      loadTasksForPage();
    }, [currentPage, itemsPerPage]); // Re-fetch when currentPage or itemsPerPage changes
    ```

### 3.3. UI Components

-   **Create `PaginationControls.tsx` Island (`islands/PaginationControls.tsx`):**
    -   Props:
        -   `currentPage: Signal<number>`
        -   `totalPages: ReadonlySignal<number>` (or `Signal<number>`)
    -   Renders:
        -   "Previous" button:
            -   Disabled if `currentPage.value <= 1`.
            -   On click: `currentPage.value--`.
        -   "Next" button:
            -   Disabled if `currentPage.value >= totalPages.value`.
            -   On click: `currentPage.value++`.
        -   Display current page info, e.g., "Page {currentPage.value} of {totalPages.value}".
    -   Conditional rendering: Do not render controls if `totalPages.value <= 1`.
    ```typescript
    // islands/PaginationControls.tsx
    import { Signal } from "@preact/signals";
    import { Button } from "../components/Button.tsx";

    interface PaginationControlsProps {
      currentPage: Signal<number>;
      totalPages: Signal<number>; // Assuming totalPages is a signal itself
    }

    export default function PaginationControls({ currentPage, totalPages }: PaginationControlsProps) {
      const handlePrevious = () => {
        if (currentPage.value > 1) {
          currentPage.value--;
        }
      };

      const handleNext = () => {
        if (currentPage.value < totalPages.value) {
          currentPage.value++;
        }
      };

      if (totalPages.value <= 1) {
        return null;
      }

      return (
        <div class="flex justify-center items-center space-x-4 my-6">
          <Button onClick={handlePrevious} disabled={currentPage.value <= 1} class="px-4 py-2">
            Previous
          </Button>
          <span class="text-gray-700">
            Page {currentPage.value} of {totalPages.value}
          </span>
          <Button onClick={handleNext} disabled={currentPage.value >= totalPages.value} class="px-4 py-2">
            Next
          </Button>
        </div>
      );
    }
    ```

-   **Integrate `PaginationControls` into `islands/TodoApp.tsx`:**
    -   Place it typically below the `AddTaskFormIsland` and above or below the `TaskList`.
    -   Pass the `currentPage` and `totalPages` signals.
    ```tsx
    // In TodoApp.tsx's return statement
    return (
      <div class="bg-white shadow-xl rounded-lg p-6 md:p-8">
        {/* ... Error display and AddTaskFormIsland ... */}
        
        {/* Add PaginationControls if not loading and there are pages */}
        {!isLoading.value && totalPages.value > 0 && (
            <PaginationControls currentPage={currentPage} totalPages={totalPages} />
        )}

        <div class="mt-8">
          {isLoading.value ? (
            <div class="text-center py-8 text-gray-500">Loading tasks...</div>
          ) : (
            <TaskList
              tasks={tasks.value}
              onToggleComplete={handleToggleComplete}
              onDeleteTask={handleDeleteTask}
            />
          )}
        </div>
        
        {/* Also render pagination controls at the bottom if preferred */}
        {!isLoading.value && totalPages.value > 1 && tasks.value.length > 0 && (
            <PaginationControls currentPage={currentPage} totalPages={totalPages} />
        )}
      </div>
    );
    ```

-   **`components/TaskList.tsx`:**
    -   The existing `TaskListProps` (tasks, onToggleComplete, onDeleteTask) will receive the paginated subset of tasks.
    -   The "No tasks yet" message should be updated to reflect that there might be no tasks *on the current page* or *at all* if `totalItems.value === 0`.
    ```tsx
    // In components/TaskList.tsx
    // ...
    if (tasks.length === 0) {
      // Consider passing totalItems or a similar prop to differentiate
      // "no tasks on this page" from "no tasks at all".
      // For now, the existing message is okay.
      return <p class="text-center text-gray-500 italic py-4">No tasks to display.</p>;
    }
    // ... rest of the component
    ```

### 3.4. User Experience for CRUD Operations

-   **Adding a Task (`handleAddTask` in `TodoApp.tsx`):**
    -   After a successful API call to create a task:
        -   To ensure the new task (often the latest) is visible, set `currentPage.value = 1;`. This will trigger the `effect` to re-fetch data for the first page.
        -   Clear the `newTaskTitle.value`.
-   **Deleting a Task (`handleDeleteTask` in `TodoApp.tsx`):**
    -   After a successful API call to delete a task:
        -   Optimistically remove the task from `tasks.value`.
        -   Then, re-fetch the current page's data to ensure consistency and get updated `totalPages` / `totalItems`.
             `fetchPaginatedTasks(currentPage.value, itemsPerPage.value).then(...)`
        -   **Edge case:** If, after optimistic removal, `tasks.value` becomes empty AND `currentPage.value > 1`, it means the last item on a page was deleted. In this case, before re-fetching, set `currentPage.value = currentPage.value - 1;`.

-   **Updating a Task (`handleToggleComplete` in `TodoApp.tsx`):**
    -   After successful API update, the existing optimistic update of `tasks.value` is likely fine. No page change is typically needed. The re-fetch mechanism is not strictly necessary for this specific action if only `completed` status changes.

### 3.5. Testing (Frontend)

-   **Manual Testing:**
    -   Navigation between pages using "Previous" and "Next" buttons.
    -   Verify correct tasks are displayed for each page.
    -   Check disabled states of "Previous"/"Next" buttons on first/last pages.
    -   Test with various scenarios: 0 tasks, 1 task, tasks fitting exactly one page, tasks spanning multiple pages.
    -   Test adding a task: verify it appears (likely on page 1).
    -   Test deleting a task: verify it's removed and current page updates correctly (especially if it was the last item on a page).
    -   Verify loading states and error messages for pagination.
-   **Component Tests (Optional):**
    -   For `PaginationControls`, test button disabled states and click handlers if logic becomes complex.

## 4. Implementation Steps (High-Level)

1.  **Backend Implementation:**
    1.  Define `PaginatedResponse` and `PaginationMeta` structs in `backend/src/models.rs`.
    2.  Define `PaginationParams` struct in `backend/src/routes.rs`.
    3.  Update `list_tasks` SQL logic (fetch limited/offset tasks and total count).
    4.  Modify the `list_tasks` handler in `backend/src/routes.rs` to parse `PaginationParams` and return `PaginatedResponse`.
    5.  Add input validation for `page` and `limit` in the handler.
    6.  Write/update backend integration tests for `GET /tasks` to cover pagination scenarios.
2.  **Frontend Implementation:**
    1.  Update `fetchTasks` (or create `fetchPaginatedTasks`) in `islands/TodoApp.tsx` (or `frontend/utils/api.ts`) to handle new request parameters and response structure.
    2.  Add/update state signals in `islands/TodoApp.tsx` (e.g., `currentPage, totalPages, totalItems`).
    3.  Modify the data-loading `effect` in `islands/TodoApp.tsx` to work with pagination.
    4.  Create and style the `islands/PaginationControls.tsx` island component.
    5.  Integrate `PaginationControls` into `islands/TodoApp.tsx`.
    6.  Implement UX improvements for add/delete task scenarios regarding current page.
    7.  Manually test all frontend pagination features thoroughly.
3.  **Review and Refine:**
    *   Ensure consistent behavior and error handling.
    *   Review code for clarity and adherence to Fresh/Axum best practices.
    *   Verify styling and responsiveness.

This plan provides a structured approach to implementing pagination. Each step should preferably be developed and tested iteratively.
