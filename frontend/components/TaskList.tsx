import { type Task } from "../types.ts";
import TaskItem from "./TaskItem.tsx";

export interface TaskListProps {
  tasks: Task[];
  onToggleComplete: (id: string) => void;
  onDeleteTask: (id: string) => void;
}

export default function TaskList({ tasks, onToggleComplete, onDeleteTask }: TaskListProps) {
  if (tasks.length === 0) {
    return <p class="text-center text-gray-500 italic py-4">No tasks yet. Add one above!</p>;
  }
  return (
    <ul class="space-y-3">
      {tasks.map((task) => (
        <TaskItem 
          key={task.id} 
          task={task} 
          onToggleComplete={onToggleComplete} 
          onDeleteTask={onDeleteTask} 
        />
      ))}
    </ul>
  );
} 