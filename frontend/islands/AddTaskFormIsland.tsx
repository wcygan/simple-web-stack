import { JSX } from "preact";
import { Signal } from "@preact/signals";

interface AddTaskFormProps {
  newTaskTitleSignal: Signal<string>;
  onAddTask: (title: string) => void;
}

export default function AddTaskFormIsland(
  { newTaskTitleSignal, onAddTask }: AddTaskFormProps,
) {
  const handleSubmit = (e: JSX.TargetedEvent<HTMLFormElement, Event>) => {
    e.preventDefault();
    onAddTask(newTaskTitleSignal.value);
  };

  return (
    <form onSubmit={handleSubmit} class="mb-8">
      <div class="flex items-stretch gap-2">
        <input
          type="text"
          value={newTaskTitleSignal.value}
          onInput={(e) =>
            newTaskTitleSignal.value = (e.target as HTMLInputElement).value}
          placeholder="What needs to be done?"
          class="flex-grow p-3 border border-gray-300 rounded-l-md focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-shadow shadow-sm hover:shadow-md"
        />
        <button
          type="submit"
          disabled={!newTaskTitleSignal.value.trim()}
          class="px-6 py-3 bg-indigo-600 text-white rounded-r-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-opacity-75 disabled:bg-gray-400 transition-colors shadow-sm hover:shadow-md"
        >
          Add Task
        </button>
      </div>
    </form>
  );
}
