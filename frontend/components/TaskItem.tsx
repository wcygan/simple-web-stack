import { type Task } from "../types.ts";
import { JSX } from "preact";

export interface TaskItemProps {
  task: Task;
  onToggleComplete: (id: string) => void;
  onDeleteTask: (id: string) => void;
}

export default function TaskItem(
  { task, onToggleComplete, onDeleteTask }: TaskItemProps,
) {
  const handleCheckboxChange = (
    e: JSX.TargetedEvent<HTMLInputElement, Event>,
  ) => {
    onToggleComplete(task.id);
  };

  const handleDeleteClick = () => {
    if (confirm(`Are you sure you want to delete "${task.title}"?`)) {
      onDeleteTask(task.id);
    }
  };

  return (
    <li
      class={`flex items-center justify-between p-4 rounded-lg transition-all duration-200 ease-in-out group shadow-sm hover:shadow-md ${
        task.completed
          ? "bg-green-50 border-l-4 border-green-500"
          : "bg-white border-l-4 border-transparent"
      }`}
    >
      <div class="flex items-center">
        <input
          type="checkbox"
          checked={task.completed}
          onChange={handleCheckboxChange}
          class="h-5 w-5 text-indigo-600 border-gray-500 rounded focus:ring-indigo-500 cursor-pointer mr-3"
        />
        <span
          class={`text-lg ${
            task.completed ? "line-through text-gray-500" : "text-gray-700"
          }`}
        >
          {task.title}
        </span>
      </div>
      <button
        onClick={handleDeleteClick}
        class="ml-4 px-3 py-1 text-sm text-red-500 hover:text-red-700 border border-red-300 hover:border-red-500 rounded-md opacity-0 group-hover:opacity-100 transition-opacity duration-200 focus:outline-none focus:ring-2 focus:ring-red-400"
        aria-label={`Delete task ${task.title}`}
      >
        Delete
      </button>
    </li>
  );
}
