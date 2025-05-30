import { effect, useSignal } from "@preact/signals";
import { type Task } from "../types.ts";
import AddTaskFormIsland from "./AddTaskFormIsland.tsx";
import TaskList from "../components/TaskList.tsx";

// API base URL - using Fresh API proxy routes for same-origin requests
const API_BASE = "/api";

// API functions for task operations
async function fetchTasks(): Promise<Task[]> {
  const response = await fetch(`${API_BASE}/tasks`);
  if (!response.ok) {
    throw new Error(`Failed to fetch tasks: ${response.statusText}`);
  }
  return await response.json();
}

async function createTask(title: string): Promise<Task> {
  const response = await fetch(`${API_BASE}/tasks`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ title }),
  });
  if (!response.ok) {
    throw new Error(`Failed to create task: ${response.statusText}`);
  }
  return await response.json();
}

async function updateTask(
  id: string,
  updates: { title?: string; completed?: boolean },
): Promise<Task> {
  const response = await fetch(`${API_BASE}/tasks/${id}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(updates),
  });
  if (!response.ok) {
    throw new Error(`Failed to update task: ${response.statusText}`);
  }
  return await response.json();
}

async function deleteTask(id: string): Promise<void> {
  const response = await fetch(`${API_BASE}/tasks/${id}`, {
    method: "DELETE",
  });
  if (!response.ok) {
    throw new Error(`Failed to delete task: ${response.statusText}`);
  }
}

export default function TodoApp() {
  const tasks = useSignal<Task[]>([]);
  const newTaskTitle = useSignal("");
  const loading = useSignal(true);
  const error = useSignal("");
  const mounted = useSignal(false);

  // Load initial tasks from the backend - only run once when component mounts
  effect(() => {
    if (mounted.value) return; // Prevent re-running

    const loadTasks = async () => {
      try {
        loading.value = true;
        error.value = "";
        const fetchedTasks = await fetchTasks();
        tasks.value = fetchedTasks;
      } catch (err) {
        console.error("Failed to load tasks:", err);
        error.value = "Failed to load tasks. Please try again.";
      } finally {
        loading.value = false;
        mounted.value = true; // Mark as mounted
      }
    };

    loadTasks();
  });

  const handleAddTask = async (title: string) => {
    if (title.trim() === "") {
      return;
    }

    try {
      error.value = "";
      const newTask = await createTask(title.trim());
      tasks.value = [newTask, ...tasks.value]; // Add to beginning to match backend ordering (newest first)
      newTaskTitle.value = "";
    } catch (err) {
      console.error("Failed to create task:", err);
      error.value = "Failed to create task. Please try again.";
    }
  };

  const handleToggleComplete = async (id: string) => {
    // Find the current task to get its current completion status
    const task = tasks.value.find((t) => t.id === id);
    if (!task) return;

    try {
      error.value = "";
      const updatedTask = await updateTask(id, { completed: !task.completed });
      tasks.value = tasks.value.map((t) => t.id === id ? updatedTask : t);
    } catch (err) {
      console.error("Failed to update task:", err);
      error.value = "Failed to update task. Please try again.";
    }
  };

  const handleDeleteTask = async (id: string) => {
    try {
      error.value = "";
      await deleteTask(id);
      tasks.value = tasks.value.filter((task) => task.id !== id);
    } catch (err) {
      console.error("Failed to delete task:", err);
      error.value = "Failed to delete task. Please try again.";
    }
  };

  return (
    <div class="bg-white shadow-xl rounded-lg p-6 md:p-8">
      {error.value && (
        <div class="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
          {error.value}
        </div>
      )}

      <AddTaskFormIsland
        newTaskTitleSignal={newTaskTitle}
        onAddTask={handleAddTask}
      />

      <div class="mt-8">
        {loading.value
          ? (
            <div class="text-center py-8">
              <div class="text-gray-500">Loading tasks...</div>
            </div>
          )
          : (
            <TaskList
              tasks={tasks.value}
              onToggleComplete={handleToggleComplete}
              onDeleteTask={handleDeleteTask}
            />
          )}
      </div>
    </div>
  );
}
