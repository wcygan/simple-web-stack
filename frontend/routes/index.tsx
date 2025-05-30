import TaskList from "../components/TaskList.tsx";
import { type Task } from "../types.ts";

const initialTasks: Task[] = [
  { id: "1", title: "Learn Fresh", completed: true },
  { id: "2", title: "Build a Todo App", completed: false },
  { id: "3", title: "Deploy to Deno Deploy", completed: false },
];

export default function Home() {
  return (
    <div class="px-4 py-8 mx-auto max-w-screen-md">
      <header class="mb-8 text-center">
        <h1 class="text-4xl font-bold text-gray-800">My Todo List</h1>
      </header>
      <main>
        <div class="bg-white shadow-md rounded-lg p-6">
          <TaskList tasks={initialTasks} />
          {/* Placeholder for Add Task Form */}
          <div class="mt-6">
            <p class="text-gray-600 italic">Add task form will go here.</p>
          </div>
        </div>
      </main>
      <footer class="mt-12 text-center text-gray-500 text-sm">
        <p>Powered by Deno Fresh & Tailwind CSS</p>
      </footer>
    </div>
  );
}
