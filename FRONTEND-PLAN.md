Okay, here's a plan for a simple integration of the Deno Fresh frontend with the Rust/Axum backend you're building. This plan assumes the backend API for tasks (CRUD operations on `/tasks`) will be available as per the backend implementation plan.

**Goal:**
Enable the Deno Fresh frontend to consume the backend API to display, create, update, and delete tasks.

**Key Technologies & Concepts:**
*   **Deno Fresh:** For frontend structure, routing, and server-side rendering (SSR) / islands.
*   **Preact Signals:** For client-side state management within islands.
*   **Fetch API:** For making HTTP requests from Fresh (both server-side in handlers and client-side in islands) to the Axum backend.
*   **CORS:** The backend needs to allow requests from the frontend's origin.
*   **Environment Variables:** To configure the backend API URL for the frontend.

---

## Plan for Frontend (Deno Fresh) Integration

**Pre-requisites:**
1.  The backend service (Axum) is running and exposing task CRUD endpoints (e.g., `GET /tasks`, `POST /tasks`, `GET /tasks/:id`, `PUT /tasks/:id`, `DELETE /tasks/:id`).
2.  The backend service is accessible from the Deno Fresh application (e.g., via `http://localhost:3000` when run locally with Docker Compose).

---

**Milestone 1: Backend API Configuration & CORS**

1.  **Backend (Axum): Configure CORS.**
    *   Ensure `tower-http` with the `cors` feature is in `backend/Cargo.toml`.
    *   In `backend/src/main.rs`, add a permissive CORS layer (or configure it more strictly for `http://localhost:8000` in development).
        ```rust
        // backend/src/main.rs
        use tower_http::cors::{Any, CorsLayer}; // Add this import

        // ... in main async fn ...
        let cors = CorsLayer::new()
            .allow_origin(Any) // Or specific_origin("http://localhost:8000")
            .allow_methods(Any) // Or Method::{GET, POST, PUT, DELETE, OPTIONS}, etc.
            .allow_headers(Any); // Or HeaderName::CONTENT_TYPE, etc.

        let app = Router::new()
            // ... your routes ...
            .layer(cors) // Apply the CORS layer
            .layer(TraceLayer::new_for_http()) // Example of another layer
            .with_state(app_state);
        ```
    *   Rebuild and restart the backend service.

2.  **Frontend (Deno Fresh): Configure Backend API URL.**
    *   **Option A (Hardcoded for simplicity, good for local Docker setup):**
        *   In `frontend/utils.ts` (or a new `frontend/config.ts`):
            ```typescript
            // frontend/utils.ts or frontend/config.ts
            export const BACKEND_API_URL = Deno.env.get("BACKEND_API_URL") || "http://localhost:3000/api"; // Assuming backend routes are under /api
            // If backend routes are at root, then "http://localhost:3000"
            // When running with docker-compose, `localhost:3000` might need to be `http://backend:3000/api` if Fresh server-side code is making the call within the Docker network.
            // For client-side calls from the browser, `http://localhost:3000/api` is correct.
            // Adjust the `BACKEND_API_URL` based on where calls are made and network setup.
            // For calls from browser (islands) directly to backend *exposed on host*:
            // export const BACKEND_API_URL = "http://localhost:3000";
            // For calls from Fresh *server-side handler* to backend *within Docker network*:
            // export const BACKEND_API_URL_SERVER_SIDE = "http://backend:3000";
            ```
        *   Consider that if Fresh's *server-side handlers* are calling the backend, they'd use the Docker service name (e.g., `http://backend:3000`). If *client-side JavaScript* (in islands) is calling, it'd use `http://localhost:3000` (as exposed by Docker Compose). For simplicity, let's assume most calls will originate from client-side islands or be proxied.
    *   Add a utility in `frontend/utils.ts` for API calls:
        ```typescript
        // frontend/utils.ts
        import { BACKEND_API_URL } from "./config.ts"; // Or define API_BASE_URL here

        export async function fetchApi<T = unknown>(path: string, options?: RequestInit): Promise<T> {
          const response = await fetch(`${BACKEND_API_URL}${path}`, options);
          if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`API Error (${response.status}): ${errorText || response.statusText}`);
          }
          if (response.headers.get("content-type")?.includes("application/json")) {
            return response.json() as Promise<T>;
          }
          return response.text() as unknown as Promise<T>; // For non-JSON or empty responses
        }
        ```

---

**Milestone 2: Displaying Tasks (Read Operation)**

1.  **Create Task Types in Frontend:**
    *   In `frontend/types.ts` (create if not exists):
        ```typescript
        // frontend/types.ts
        export interface Task {
          id: string; // UUID
          title: string;
          completed: boolean;
          created_at?: string; // ISO date string
          updated_at?: string; // ISO date string
        }
        ```

2.  **Create a New Route for Tasks (`frontend/routes/tasks.tsx`):**
    ```tsx
    // frontend/routes/tasks.tsx
    import { Head } from "fresh/runtime.ts";
    import { Handlers, PageProps } from "fresh/server.ts";
    import { useSignal } from "@preact/signals";
    import TaskList from "../islands/TaskList.tsx";
    import AddTaskForm from "../islands/AddTaskForm.tsx";
    import { Task } from "../types.ts";
    import { fetchApi } from "../utils.ts"; // Or from ./config.ts or a dedicated api.ts

    export const handler: Handlers<Task[] | null> = {
      async GET(_req, ctx) {
        try {
          const tasks = await fetchApi<Task[]>("/tasks"); // Assumes backend API is at /tasks
          return ctx.render(tasks);
        } catch (error) {
          console.error("Failed to fetch tasks:", error);
          return ctx.render(null); // Render with error or empty state
        }
      },
    };

    export default function TasksPage({ data }: PageProps<Task[] | null>) {
      const tasksSignal = useSignal<Task[]>(data || []);

      if (data === null && !tasksSignal.value.length) {
          return (
            <>
              <Head><title>Tasks - Error</title></Head>
              <div class="p-4 mx-auto max-w-screen-md">
                <h1 class="text-2xl font-bold">Task Manager</h1>
                <p class="text-red-500">Failed to load tasks. Please try again later.</p>
              </div>
            </>
          );
      }

      return (
        <>
          <Head>
            <title>Tasks</title>
          </Head>
          <div class="p-4 mx-auto max-w-screen-md">
            <img
              src="/logo.svg" // Assuming general layout desire
              width="64"
              height="64"
              alt="App Logo"
              class="mx-auto mb-4"
            />
            <h1 class="text-2xl font-bold text-center mb-6">Task Manager</h1>
            <AddTaskForm tasksSignal={tasksSignal} />
            <TaskList tasksSignal={tasksSignal} />
          </div>
        </>
      );
    }
    ```

3.  **Create `TaskList` Island (`frontend/islands/TaskList.tsx`):**
    ```tsx
    // frontend/islands/TaskList.tsx
    import { Signal } from "@preact/signals";
    import { Task } from "../types.ts";
    import { fetchApi } from "../utils.ts";
    import { Button } from "../components/Button.tsx"; // Assuming Button.tsx component exists

    interface TaskListProps {
      tasksSignal: Signal<Task[]>;
    }

    export default function TaskList({ tasksSignal }: TaskListProps) {
      const toggleComplete = async (task: Task) => {
        try {
          const updatedTask = await fetchApi<Task>(`/tasks/${task.id}`, {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ completed: !task.completed }),
          });
          tasksSignal.value = tasksSignal.value.map((t) =>
            t.id === updatedTask.id ? updatedTask : t
          );
        } catch (error) {
          console.error("Failed to update task:", error);
          // Add user feedback, e.g., toast notification
        }
      };

      const deleteTask = async (taskId: string) => {
        if (!confirm("Are you sure you want to delete this task?")) return;
        try {
          await fetchApi(`/tasks/${taskId}`, { method: "DELETE" });
          tasksSignal.value = tasksSignal.value.filter((t) => t.id !== taskId);
        } catch (error) {
          console.error("Failed to delete task:", error);
          // Add user feedback
        }
      };

      if (tasksSignal.value.length === 0) {
        return <p class="text-center text-gray-500 mt-8">No tasks yet. Add one above!</p>;
      }

      return (
        <ul class="list-disc pl-5 mt-6 space-y-2">
          {tasksSignal.value.map((task) => (
            <li key={task.id} class="flex items-center justify-between p-2 border-b">
              <span
                class={`cursor-pointer ${task.completed ? "line-through text-gray-500" : ""}`}
                onClick={() => toggleComplete(task)}
              >
                {task.title}
              </span>
              <div>
                <Button
                    onClick={() => deleteTask(task.id)}
                    class="ml-2 px-2 py-1 bg-red-500 hover:bg-red-700 text-white text-xs rounded"
                >
                    Delete
                </Button>
              </div>
            </li>
          ))}
        </ul>
      );
    }
    ```

---

**Milestone 3: Creating Tasks (Create Operation)**

1.  **Create `AddTaskForm` Island (`frontend/islands/AddTaskForm.tsx`):**
    ```tsx
    // frontend/islands/AddTaskForm.tsx
    import { Signal, useSignal } from "@preact/signals";
    import { Button } from "../components/Button.tsx";
    import { Task } from "../types.ts";
    import { fetchApi } from "../utils.ts";

    interface AddTaskFormProps {
      tasksSignal: Signal<Task[]>;
    }

    export default function AddTaskForm({ tasksSignal }: AddTaskFormProps) {
      const title = useSignal("");
      const error = useSignal<string | null>(null);
      const isLoading = useSignal(false);

      const handleSubmit = async (e: Event) => {
        e.preventDefault();
        if (!title.value.trim()) {
          error.value = "Title cannot be empty.";
          return;
        }
        isLoading.value = true;
        error.value = null;
        try {
          const newTask = await fetchApi<Task>("/tasks", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ title: title.value }),
          });
          tasksSignal.value = [...tasksSignal.value, newTask];
          title.value = ""; // Clear input
        } catch (err) {
          console.error("Failed to add task:", err);
          error.value = err.message || "Failed to add task.";
        } finally {
          isLoading.value = false;
        }
      };

      return (
        <form onSubmit={handleSubmit} class="flex gap-2 mb-4">
          <input
            type="text"
            value={title.value}
            onInput={(e) => title.value = (e.target as HTMLInputElement).value}
            placeholder="New task title"
            class="flex-grow p-2 border rounded"
            disabled={isLoading.value}
          />
          <Button type="submit" disabled={isLoading.value}>
            {isLoading.value ? "Adding..." : "Add Task"}
          </Button>
          {error.value && <p class="text-red-500 text-sm mt-1">{error.value}</p>}
        </form>
      );
    }
    ```

---

**Milestone 4: Updating & Deleting Tasks (Update/Delete Operations)**

*   These are already partially implemented in `TaskList.tsx` (Milestone 2, step 3).
*   **Refine UI for Updates:** Could add an "Edit" button to `TaskItem.tsx` (or inline if simpler) that reveals an input field for changing the title. This would involve:
    *   A signal for `isEditing` state per task.
    *   An input field bound to a temporary title signal.
    *   "Save" and "Cancel" buttons.
    *   `PUT` request to `/tasks/:id` with the new title.

---

**Milestone 5: UI/UX Refinements and Error Handling**

1.  **Loading States:**
    *   Show loading indicators (spinners, disabled buttons) while API calls are in progress (partially done in `AddTaskForm`).
2.  **Error Display:**
    *   Provide clear visual feedback to the user if an API call fails (e.g., toast notifications, inline error messages). Improve on current `console.error`.
3.  **Styling:**
    *   Utilize Tailwind CSS (already set up with `plugin-tailwind`) to make the UI more appealing. Create a consistent look and feel. The provided examples give a basic structure.
4.  **Accessibility:**
    *   Ensure forms and interactive elements are accessible (e.g., proper labels, keyboard navigation).
5.  **Confirmations:**
    *   Use `confirm()` or custom modal dialogs for destructive actions like task deletion (already done with `confirm()` in `TaskList`).

---

**Milestone 6: Testing (Frontend)**

1.  **Component Tests:**
    *   Write Deno tests for new components/islands (`TaskList.tsx`, `AddTaskForm.tsx`) in `frontend/tests/`.
    *   Test prop handling, basic rendering, and simulated interactions if possible (mocking `fetch`).
    *   Example (`frontend/tests/TaskList_test.ts`):
        ```typescript
        // frontend/tests/TaskList_test.ts
        import { assertEquals, assertExists } from "jsr:@std/assert";
        import { 하는 Signal, signal } from "@preact/signals";
        import TaskList from "../islands/TaskList.tsx";
        import { Task } from "../types.ts";
        // Mock fetch, etc. for more advanced tests.
        // For simple rendering tests:
        Deno.test("TaskList renders with empty tasks", () => {
          const tasksSignal: Signal<Task[]> = signal([]);
          const comp = TaskList({ tasksSignal }); // Note: This won't fully render in Deno test env without a DOM
          assertExists(comp);
          // More meaningful tests would involve a virtual DOM or testing utilities
          // For now, just check that the component function can be called.
        });
        ```
        *(Testing Fresh islands can be tricky without a browser-like environment. Focus on utility functions and prop validation for now, or explore tools like Preact Testing Library if more in-depth component testing is needed).*

2.  **Manual End-to-End Testing:**
    *   Run the full stack with `deno task up` (which should run `docker-compose up`).
    *   Navigate to `http://localhost:8000/tasks` and test all functionalities.

---

**Running the Full Stack:**

Ensure your root `deno.json`'s `up` task correctly starts both services:
`"up": "docker compose up -d --build"`

Then run:
`deno task up`

Access the frontend at `http://localhost:8000/tasks` (or whatever route you choose).
The backend API will be at `http://localhost:3000`.

This plan provides a clear path to integrate your Deno Fresh frontend with the Rust backend, focusing on simplicity and leveraging Fresh's features.