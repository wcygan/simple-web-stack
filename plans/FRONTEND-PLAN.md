# Frontend Development Plan: Todo App (Deno Fresh 2)

This plan outlines the steps to build the user interface for a Todo List application using Deno Fresh 2. It focuses on an iterative approach, building UI components first, adding interactivity with Islands and Preact Signals, and ensuring each step is testable. Backend integration will be addressed in a later phase.

## I. Project Setup & Initial Structure (Fresh 2 Idioms)

**Goal:** Initialize a new Deno Fresh 2 project and configure it for development, including Tailwind CSS.

**Steps:**

1.  **Initialize Fresh Project:**
    * DONE.

2.  **Review and Understand Fresh 2 Project Structure:**
    *   Action: Familiarize yourself with `dev.ts` (development server, build configurations) and `main.ts` (production server setup, middleware, `fsRoutes`).
    *   Note how plugins (like Tailwind) are configured in `dev.ts`.
    *   Verification: Conceptual understanding of the roles of `dev.ts` and `main.ts`.

3.  **Basic App Layout (`routes/_app.tsx`):**
    *   Action:
        *   Create/modify `routes/_app.tsx`.
        *   Set up the basic HTML structure (html, head, body).
        *   Include a link to the Tailwind CSS output file (e.g., `/styles.css` if generated into `static/styles.css` by the Tailwind plugin).
        *   Ensure `ctx.Component` (or `props.Component` in Fresh 1.x style, prefer `FreshContext` and `ctx.Component` for Fresh 2) is rendered within the body.
    *   Verification:
        *   The basic HTML structure is applied to all pages.
        *   Tailwind base styles (if any) are visibly affecting the default page.

4.  **Configure Tailwind CSS:**
    *   Action:
        *   Ensure `tailwind.config.ts` is present.
        *   Verify the Tailwind plugin is correctly configured in `dev.ts` (usually done by `fresh init`).
        *   Create a basic CSS file (e.g., `static/styles.css` or wherever the Tailwind plugin outputs) that includes Tailwind directives:
            ```css
            @tailwind base;
            @tailwind components;
            @tailwind utilities;
            ```
        *   Link this CSS file in `routes/_app.tsx`.
    *   Verification:
        *   Tailwind utility classes can be applied to elements in `routes/index.tsx` and changes are reflected in the browser.
        *   No console errors related to Tailwind.

## II. Static UI - Displaying Tasks

**Goal:** Create the basic UI for displaying a list of tasks. Initially, this will be static or use hardcoded data.

**Steps:**

1.  **Task Item Component (`components/TaskItem.tsx`):**
    *   Action: Create a non-island component to represent a single task.
        *   It should display:
            *   Task title (text).
            *   Completed status (e.g., a checkbox or a visual indicator).
            *   Delete button placeholder (styling only for now).
        *   Props: `task: { id: string, title: string, completed: boolean }`.
    *   Verification:
        *   Component can be imported and rendered within `routes/index.tsx` with sample data.
        *   Tailwind CSS should be used for styling.

2.  **Task List Component (`components/TaskList.tsx`):**
    *   Action: Create a non-island component to display a list of `TaskItem` components.
        *   Props: `tasks: Array<{ id: string, title: string, completed: boolean }>`.
        *   It should iterate over the `tasks` prop and render a `TaskItem` for each.
    *   Verification:
        *   Can be rendered in `routes/index.tsx` with a hardcoded array of task objects.
        *   Displays multiple tasks correctly.

3.  **Main Page UI (`routes/index.tsx`):**
    *   Action:
        *   Modify `routes/index.tsx` to be the main page of the Todo app.
        *   Include a title (e.g., "My Todo List").
        *   Use the `TaskList` component with some hardcoded initial tasks.
        *   Add a placeholder for the "Add Task" form.
    *   Verification:
        *   The main page displays the title and the list of hardcoded tasks.
        *   The page is styled using Tailwind CSS.

## III. Static UI - Adding New Tasks

**Goal:** Create the static UI for the "Add Task" form.

**Steps:**

1.  **Add Task Form Component (`components/AddTaskForm.tsx`):**
    *   Action: Create a non-island component for the task input form.
        *   It should contain:
            *   A text input field for the task title.
            *   A submit button (e.g., "Add Task").
        *   For now, the form won't have actual submission logic.
    *   Verification:
        *   Component can be imported and rendered in `routes/index.tsx`.
        *   The form elements are visible and styled with Tailwind.

2.  **Integrate Add Task Form into Main Page:**
    *   Action: Add the `AddTaskForm` component to `routes/index.tsx`.
    *   Verification: The main page now shows the "Add Task" form alongside the task list.

## IV. Client-Side Interactivity with Islands & Signals

**Goal:** Make the application interactive using Fresh Islands and Preact Signals for state management.

**Steps:**

1.  **Todo App State Island (`islands/TodoApp.tsx`):**
    *   Action:
        *   Create an island that will manage the overall state of the todo list (tasks array, new task input).
        *   Use Preact Signals (`useSignal`) to manage:
            *   `tasks`: An array of task objects. Initialize with a few sample tasks.
            *   `newTaskTitle`: A string for the new task input field.
        *   This island will render `TaskList` and `AddTaskForm`, passing down signals and callbacks as props.
    *   Verification:
        *   Replace the static `TaskList` and `AddTaskForm` in `routes/index.tsx` with this `TodoApp` island.
        *   The page should still display the initial hardcoded tasks (now managed by the island's signal).

2.  **Making `AddTaskForm` Interactive (within `islands/TodoApp.tsx` or as a new island):**
    *   Action:
        *   Modify `AddTaskForm` (or create an `islands/AddTaskFormIsland.tsx` if preferred for separation) to take `newTaskTitle` signal and an `onAddTask` callback as props.
        *   The text input should be two-way bound to the `newTaskTitle` signal.
        *   The form submission (or button click) should:
            *   Prevent default form submission.
            *   Call the `onAddTask` callback passed from `TodoApp` island, which will update the `tasks` signal.
            *   Clear the `newTaskTitle` signal.
    *   Props for `AddTaskForm`:
        *   `newTaskTitleSignal: Signal<string>`
        *   `handleAddTask: (title: string) => void`
    *   Logic in `TodoApp` island:
        *   `handleAddTask` function: Creates a new task object, adds it to the `tasks` signal's array.
    *   Verification:
        *   Typing in the input field updates the `newTaskTitle` signal.
        *   Clicking "Add Task" adds a new task to the list displayed by `TaskList` and clears the input.
        *   The list of tasks dynamically updates.

3.  **Making `TaskItem` Interactive (within `islands/TodoApp.tsx` or as `islands/TaskItemIsland.tsx`):**
    *   Action:
        *   Modify `TaskItem` (or create `islands/TaskItemIsland.tsx`) to handle interactions.
        *   Props for `TaskItem`:
            *   `task: Signal<{ id: string, title: string, completed: boolean }>` (or pass individual task props and callbacks)
            *   `onToggleComplete: (id: string) => void`
            *   `onDeleteTask: (id: string) => void`
        *   The checkbox should reflect `task.value.completed` and call `onToggleComplete` on change.
        *   The delete button should call `onDeleteTask` on click.
    *   Logic in `TodoApp` island:
        *   `handleToggleComplete` function: Finds the task by ID in the `tasks` signal and toggles its `completed` status.
        *   `handleDeleteTask` function: Filters the task by ID from the `tasks` signal's array.
    *   Verification:
        *   Clicking a task's checkbox toggles its completed status visually (e.g., strikethrough text, checkbox state).
        *   Clicking a task's delete button removes it from the list.
        *   The `tasks` signal in `TodoApp` island correctly reflects these changes.

## V. Styling and Final UI Touches

**Goal:** Ensure the application is well-styled and user-friendly.

**Steps:**

1.  **Refine Tailwind CSS Styling:**
    *   Action: Go through all components and islands, applying Tailwind classes to achieve a clean, modern, and consistent look and feel.
    *   Consider responsive design.
    *   Style active/completed states for tasks.
    *   Style form inputs, buttons, and overall layout.
    *   Verification: The application is visually appealing and responsive on different screen sizes (manual check).

2.  **Empty State for Task List:**
    *   Action: Modify `TaskList` (or `TodoApp` island) to display a message when there are no tasks (e.g., "No tasks yet! Add one above.").
    *   Verification: When all tasks are deleted, or if the app starts with no tasks, the empty state message is shown.

3.  **Basic Error Handling (Client-Side):**
    *   Action:
        *   For `AddTaskForm`, add simple client-side validation (e.g., prevent adding an empty task title). Display a subtle message if validation fails.
        *   This is a placeholder for more robust error handling later.
    *   Verification:
        *   Attempting to add a task with an empty title shows a message and doesn't add the task.

## VI. Preparation for Backend Integration (Conceptual)

**Goal:** Ensure the frontend is structured in a way that will make backend integration smoother. This phase is primarily about structuring props and callbacks.

**Steps:**

1.  **Review Prop Contracts:**
    *   Action: Review the props for all islands and components. Ensure they are well-defined for future data flow from/to a backend API.
    *   For example, `onAddTask`, `onToggleComplete`, `onDeleteTask` will eventually trigger API calls.
    *   Verification: Props are clear and represent the data and actions needed.

2.  **Identify API Interaction Points:**
    *   Action: Mentally (or via comments in code) mark where API calls will be needed:
        *   Fetching initial tasks.
        *   Adding a new task.
        *   Updating a task's completion status.
        *   Deleting a task.
    *   Verification: Clear understanding of where frontend actions will translate to backend requests.

## VII. Testing and Verification (Manual for UI Phase)

**Goal:** Continuously verify the functionality and appearance of the UI.

**Throughout all phases:**

*   **Manual Testing:**
    *   After each significant change, manually test the UI in the browser.
    *   Check for:
        *   Correct rendering of components.
        *   Interactivity (button clicks, form inputs).
        *   State updates (signals reflecting changes).
        *   Visual styling (Tailwind classes applied correctly).
        *   Console errors in the browser developer tools.
*   **Code Review (Self or Peer):**
    *   Ensure code follows Fresh 2 idioms.
    *   Check for clarity and maintainability.

This plan provides a structured approach to building the frontend. Each major section (I-VI) and its steps should result in a testable increment of the application.
The next major phase will be integrating this frontend with the Rust/Axum backend API.
