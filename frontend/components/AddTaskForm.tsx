import { JSX } from "preact";

export function AddTaskForm() {
  return (
    <form class="mt-4 mb-6">
      <div class="flex items-center gap-2">
        <input
          type="text"
          name="title"
          placeholder="What needs to be done?"
          class="flex-grow p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <button
          type="submit"
          class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
        >
          Add Task
        </button>
      </div>
    </form>
  );
} 