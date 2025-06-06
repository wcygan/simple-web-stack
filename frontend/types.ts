export interface Task {
  id: string; // UUID
  title: string;
  completed: boolean;
  user_id: string; // UUID
  created_at?: string; // ISO date string
  updated_at?: string; // ISO date string
}

export interface PaginationParams {
  page?: number;
  limit?: number;
  offset?: number;
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    page_size: number;
    total_items: number;
    total_pages: number;
    has_next: boolean;
    has_previous: boolean;
    // Legacy compatibility
    total?: number;
    limit?: number;
    has_prev?: boolean;
  };
}

export interface SearchParams {
  q?: string;         // Search query for title
  status?: string;    // Filter by status: "completed", "pending", "all"
}

export interface TaskQueryParams extends PaginationParams, SearchParams {
  page_size?: number;  // Map to backend's page_size parameter
  sort_by?: string;    // Sort column
  sort_order?: 'asc' | 'desc'; // Sort direction
}

// Auth Types
export interface User {
  id: string;
  email: string;
  created_at?: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  email: string;
  password: string;
}

export interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
}
