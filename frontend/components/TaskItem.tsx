import { type Task } from "../types.ts";

export interface TaskItemProps {
  task: Task;
}

export default function TaskItem({ task }: TaskItemProps) {
  return (
    <li class="flex items-center justify-between p-2 border-b border-gray-200">
      <div class="flex items-center">
        <input
          type="checkbox"
          checked={task.completed}
          class="mr-2 h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
          // For now, purely visual, no interactivity
          readOnly 
        />
        <span class={`${task.completed ? "line-through text-gray-500" : "text-gray-800"}`}>
          {task.title}
        </span>
      </div>
      <button class="px-2 py-1 text-sm text-red-500 hover:text-red-700">
        Delete
      </button>
    </li>
  );
} 