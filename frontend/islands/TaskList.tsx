import { Signal, signal } from "@preact/signals";
import { Task } from "../types.ts";
import { Button } from "../components/Button.tsx";

interface TaskListProps {
  tasksSignal: Signal<Task[]>;
}

// Mocked data for now, as per user request
const initialTasks: Task[] = [
  {
    id: "1",
    title: "Learn Deno Fresh",
    completed: true,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  },
  {
    id: "2",
    title: "Build a task app",
    completed: false,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  },
  {
    id: "3",
    title: "Deploy to the cloud",
    completed: false,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  },
];

export default function TaskList({ tasksSignal }: TaskListProps) {
  // If tasksSignal is not initialized with data, use mock data.
  // This setup allows the page to later provide real data through the signal.
  if (tasksSignal.value.length === 0) {
    tasksSignal.value = initialTasks;
  }

  const toggleComplete = (task: Task) => {
    // Mocked behavior: toggle completion status locally
    tasksSignal.value = tasksSignal.value.map((t) =>
      t.id === task.id ? { ...t, completed: !t.completed } : t
    );
    console.log(`Mocked: Toggle complete for task ${task.id}`);
  };

  const deleteTask = (taskId: string) => {
    if (!confirm("Are you sure you want to delete this task?")) return;
    // Mocked behavior: delete task locally
    tasksSignal.value = tasksSignal.value.filter((t) => t.id !== taskId);
    console.log(`Mocked: Delete task ${taskId}`);
  };

  if (tasksSignal.value.length === 0) {
    return (
      <p class="text-center text-gray-500 mt-8">No tasks yet. Add one above!</p>
    );
  }

  return (
    <ul class="list-none pl-0 mt-6 space-y-2">
      {tasksSignal.value.map((task) => (
        <li
          key={task.id}
          class="flex items-center justify-between p-3 bg-white border border-gray-200 rounded-lg shadow-sm"
        >
          <span
            class={`cursor-pointer ${
              task.completed ? "line-through text-gray-500" : "text-gray-800"
            }`}
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
