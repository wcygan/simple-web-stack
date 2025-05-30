// import { Head } from "fresh/runtime"; // <Head> component is removed in Fresh 2
import { PageProps } from "fresh"; // Corrected: import from 'fresh' directly
import { useSignal } from "@preact/signals";
import TaskList from "../islands/TaskList.tsx";
import AddTaskForm from "../islands/AddTaskForm.tsx";
import { Task } from "../types.ts";

// Mocked initial data for the page, similar to TaskList's initial data for consistency if needed
// However, TaskList island already has its own initial mock data if tasksSignal is empty.
const pageLevelMockTasks: Task[] = [
  // { id: "p1", title: "Page Load Task 1", completed: false },
  // { id: "p2", title: "Page Load Task 2", completed: true },
];

export default function TasksPage({ data }: PageProps<null>) { // Data from handler will be null for now
  // Initialize tasksSignal. TaskList will populate with its own mock data if this is empty.
  const tasksSignal = useSignal<Task[]>(data || pageLevelMockTasks);

  return (
    <>
      {/* <Head>
        <title>My Tasks</title>
        <meta name="description" content="A simple application to manage your tasks." />
        <link rel="stylesheet" href="/styles.css" /> 
      </Head> */}
      <div class="p-4 mx-auto max-w-screen-md">
        <header class="text-center my-8">
          <img
            src="/logo.svg" // Assuming logo.svg is in static/
            width="80"
            height="80"
            alt="Application Logo"
            class="mx-auto mb-4"
          />
          <h1 class="text-4xl font-bold text-gray-800">Task Manager</h1>
          <p class="text-lg text-gray-600">Organize your day, one task at a time.</p>
        </header>
        
        <main>
          <section aria-labelledby="add-task-heading" class="mb-8">
            <h2 id="add-task-heading" class="sr-only">Add New Task</h2> {/* For accessibility */}
            <AddTaskForm tasksSignal={tasksSignal} />
          </section>

          <section aria-labelledby="task-list-heading">
            <h2 id="task-list-heading" class="text-2xl font-semibold text-gray-700 mb-4">Your Tasks</h2>
            <TaskList tasksSignal={tasksSignal} />
          </section>
        </main>

        <footer class="text-center mt-12 py-4 border-t border-gray-200">
          <p class="text-sm text-gray-500">
            &copy; {new Date().getFullYear()} Simple Web Stack, Inc. All rights reserved.
          </p>
        </footer>
      </div>
    </>
  );
} 