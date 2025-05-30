import { JSX } from "preact";
import { useSignal, Signal } from "@preact/signals";
import { type Task } from "../types.ts";
import { nanoid } from "npm:nanoid@5.0.7"; // For generating unique IDs

// Sample initial tasks - in a real app, this would come from an API
const initialTasks: Task[] = [
  { id: nanoid(5), title: "Learn about Fresh Islands", completed: true },
  { id: nanoid(5), title: "Use Preact Signals", completed: false },
  { id: nanoid(5), title: "Style with Tailwind CSS", completed: false },
];

export default function TodoApp() {
  const tasks = useSignal<Task[]>(initialTasks);
  const newTaskTitle = useSignal<string>("");

  const handleAddTask = () => {
    const title = newTaskTitle.value.trim();
    if (title) {
      const newTask: Task = {
        id: nanoid(5), // Generate a unique ID
        title,
        completed: false,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };
      tasks.value = [...tasks.value, newTask];
      newTaskTitle.value = ""; // Clear the input field
    } else {
      // Basic validation: alert or log, could be more sophisticated
      alert("Task title cannot be empty!");
    }
  };

  const handleToggleComplete = (id: string) => {
    tasks.value = tasks.value.map((task) =>
      task.id === id ? { ...task, completed: !task.completed, updated_at: new Date().toISOString() } : task
    );
  };

  const handleDeleteTask = (id: string) => {
    tasks.value = tasks.value.filter((task) => task.id !== id);
  };

  return (
    <div class="bg-white shadow-md rounded-lg p-6">
      <AddTaskFormIsland newTaskTitleSignal={newTaskTitle} onAddTask={handleAddTask} />
      <TaskListIsland tasksSignal={tasks} onToggleComplete={handleToggleComplete} onDeleteTask={handleDeleteTask} />
    </div>
  );
}

// --- Sub-Islands or Interactive Components ---

// Making AddTaskForm interactive (as a component within TodoApp or separate island)
interface AddTaskFormIslandProps {
  newTaskTitleSignal: Signal<string>;
  onAddTask: () => void;
}

function AddTaskFormIsland({ newTaskTitleSignal, onAddTask }: AddTaskFormIslandProps) {
  const handleSubmit = (e: JSX.TargetedEvent<HTMLFormElement, Event>) => {
    e.preventDefault();
    onAddTask();
  };

  return (
    <form onSubmit={handleSubmit} class="mt-4 mb-6">
      <div class="flex items-center gap-2">
        <input
          type="text"
          name="title"
          placeholder="What needs to be done?"
          class="flex-grow p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          value={newTaskTitleSignal.value}
          onInput={(e) => newTaskTitleSignal.value = (e.target as HTMLInputElement).value}
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

// Making TaskList and TaskItem interactive
interface TaskListIslandProps {
  tasksSignal: Signal<Task[]>;
  onToggleComplete: (id: string) => void;
  onDeleteTask: (id: string) => void;
}

function TaskListIsland({ tasksSignal, onToggleComplete, onDeleteTask }: TaskListIslandProps) {
  if (tasksSignal.value.length === 0) {
    return <p class="text-center text-gray-500 italic py-4">No tasks yet! Add one above.</p>;
  }
  return (
    <ul class="space-y-3">
      {tasksSignal.value.map((task) => (
        <TaskItemIsland
          key={task.id}
          task={task} // Pass the plain task object
          onToggleComplete={() => onToggleComplete(task.id)}
          onDeleteTask={() => onDeleteTask(task.id)}
        />
      ))}
    </ul>
  );
}

interface TaskItemIslandProps {
  task: Task; // Individual task object, not a signal here unless absolutely necessary for deep reactivity
  onToggleComplete: () => void;
  onDeleteTask: () => void;
}

function TaskItemIsland({ task, onToggleComplete, onDeleteTask }: TaskItemIslandProps) {
  return (
    <li class={`flex items-center justify-between p-3 rounded-md transition-colors duration-150 ease-in-out ${task.completed ? 'bg-green-50 text-gray-500 line-through' : 'bg-gray-50'}`}>
      <div class="flex items-center">
        <input
          type="checkbox"
          checked={task.completed}
          onChange={onToggleComplete}
          class="h-5 w-5 text-blue-500 border-gray-300 rounded focus:ring-blue-400 mr-3 cursor-pointer"
          id={`task-${task.id}`}
        />
        <label htmlFor={`task-${task.id}`} class="cursor-pointer" >
          {task.title}
        </label>
      </div>
      <button
        onClick={onDeleteTask}
        class="px-3 py-1 bg-red-500 text-white text-xs font-semibold rounded-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-opacity-50"
      >
        Delete
      </button>
    </li>
  );
} 