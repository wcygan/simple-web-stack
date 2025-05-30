export interface Task {
  id: string; // UUID
  title: string;
  completed: boolean;
  created_at?: string; // ISO date string
  updated_at?: string; // ISO date string
} 