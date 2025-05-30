import { type Task } from "../types.ts";
import TaskItem from "./TaskItem.tsx";

export interface TaskListProps {
  tasks: Task[];
}

export default function TaskList({ tasks }: TaskListProps) {
  if (tasks.length === 0) {
    return <p class="text-gray-500 italic">No tasks yet. Add one below!</p>;
  }
  return (
    <ul class="list-none p-0 m-0">
      {tasks.map((task) => (
        <TaskItem key={task.id} task={task} />
      ))}
    </ul>
  );
} 