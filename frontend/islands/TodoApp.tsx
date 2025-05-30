import { useSignal } from "@preact/signals";
import { type Task } from "../types.ts";
import { nanoid } from "npm:nanoid@5.0.7";
import AddTaskFormIsland from "./AddTaskFormIsland.tsx";
import TaskList from "../components/TaskList.tsx";

// Sample initial tasks - in a real app, this would come from an API
const initialTasks: Task[] = [
  { id: nanoid(5), title: "Learn about Fresh Islands", completed: true },
  { id: nanoid(5), title: "Use Preact Signals", completed: false },
  { id: nanoid(5), title: "Style with Tailwind CSS", completed: false },
];

export default function TodoApp() {
  const tasks = useSignal<Task[]>([
    { id: nanoid(5), title: "Learn Fresh", completed: true },
    { id: nanoid(5), title: "Build a Todo App", completed: false },
    { id: nanoid(5), title: "Deploy to Deno Deploy", completed: false },
  ]);
  const newTaskTitle = useSignal("");

  const handleAddTask = (title: string) => {
    if (title.trim() === "") {
      return;
    }
    tasks.value = [
      ...tasks.value,
      { id: nanoid(5), title: title.trim(), completed: false },
    ];
    newTaskTitle.value = "";
  };

  const handleToggleComplete = (id: string) => {
    tasks.value = tasks.value.map((task) =>
      task.id === id ? { ...task, completed: !task.completed } : task
    );
  };

  const handleDeleteTask = (id: string) => {
    tasks.value = tasks.value.filter((task) => task.id !== id);
  };

  return (
    <div class="bg-white shadow-xl rounded-lg p-6 md:p-8">
      <AddTaskFormIsland
        newTaskTitleSignal={newTaskTitle}
        onAddTask={handleAddTask}
      />
      <div class="mt-8">
        <TaskList
          tasks={tasks.value}
          onToggleComplete={handleToggleComplete}
          onDeleteTask={handleDeleteTask}
        />
      </div>
    </div>
  );
} 