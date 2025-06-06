import { Signal, useSignal } from "@preact/signals";
import { Button } from "../components/Button.tsx";
import { Task } from "../types.ts";

interface AddTaskFormProps {
  tasksSignal: Signal<Task[]>;
}

export default function AddTaskForm({ tasksSignal }: AddTaskFormProps) {
  const title = useSignal("");
  const error = useSignal<string | null>(null);
  const isLoading = useSignal(false); // Mock loading state

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    if (!title.value.trim()) {
      error.value = "Title cannot be empty.";
      return;
    }
    isLoading.value = true;
    error.value = null;

    // Mocked behavior: Add task locally
    const newTask: Task = {
      id: crypto.randomUUID(), // Use crypto.randomUUID for a mock ID
      title: title.value.trim(),
      completed: false,
      user_id: "00000000-0000-0000-0000-000000000000",
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    tasksSignal.value = [...tasksSignal.value, newTask];
    title.value = ""; // Clear input
    console.log("Mocked: Add task", newTask);

    // Simulate API call delay
    await new Promise((resolve) => setTimeout(resolve, 500));
    isLoading.value = false;
  };

  return (
    <form onSubmit={handleSubmit} class="flex flex-col sm:flex-row gap-2 mb-4">
      <input
        type="text"
        value={title.value}
        onInput={(e) => {
          title.value = (e.target as HTMLInputElement).value;
          if (error.value) {
            error.value = null; // Clear error on input
          }
        }}
        placeholder="Enter new task title..."
        class="flex-grow p-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
        disabled={isLoading.value}
      />
      <Button
        type="submit"
        disabled={isLoading.value || !title.value.trim()}
        class="w-full sm:w-auto"
      >
        {isLoading.value ? "Adding..." : "Add Task"}
      </Button>
      {error.value && (
        <p class="text-red-500 text-sm mt-1 w-full">{error.value}</p>
      )}
    </form>
  );
}
