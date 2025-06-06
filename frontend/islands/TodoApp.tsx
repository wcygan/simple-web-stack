import { effect, useSignal } from "@preact/signals";
import { useEffect, useRef } from "preact/hooks";
import { type Task, type SearchParams, type TaskQueryParams, type PaginatedResponse } from "../types.ts";
import AddTaskFormIsland from "./AddTaskFormIsland.tsx";
import TaskList from "../components/TaskList.tsx";
import { SearchForm } from "../components/SearchForm.tsx";

// API base URL - using Fresh API proxy routes for same-origin requests
const API_BASE = "/api";

// API functions for task operations
async function fetchTasks(searchParams?: TaskQueryParams): Promise<PaginatedResponse<Task>> {
  let url = `${API_BASE}/tasks`;
  
  if (searchParams) {
    const params = new URLSearchParams();
    
    // Add pagination params
    if (searchParams.page) params.append('page', searchParams.page.toString());
    if (searchParams.page_size) params.append('page_size', searchParams.page_size.toString());
    if (searchParams.sort_by) params.append('sort_by', searchParams.sort_by);
    if (searchParams.sort_order) params.append('sort_order', searchParams.sort_order);
    
    // Add search params
    if (searchParams.q && searchParams.q.trim()) params.append('q', searchParams.q.trim());
    if (searchParams.status && searchParams.status !== 'all') params.append('status', searchParams.status);
    
    const queryString = params.toString();
    if (queryString) {
      url += `?${queryString}`;
    }
  }
  
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch tasks: ${response.statusText}`);
  }
  return await response.json();
}

// Legacy function for backwards compatibility
async function fetchAllTasks(): Promise<Task[]> {
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
  
  // Search and pagination state
  const searchParams = useSignal<SearchParams>({ q: "", status: "all" });
  const queryParams = useSignal<TaskQueryParams>({ page: 1, page_size: 10 });
  const totalTasks = useSignal(0);
  const totalPages = useSignal(1);
  const currentPage = useSignal(1);
  
  // Ref to prevent infinite loops
  const isLoadingRef = useRef(false);
  
  // Debounce timeout for search
  let searchTimeout: number | undefined;

  // Load tasks with search/pagination - runs when queryParams change
  const loadTasks = async () => {
    console.log("loadTasks called with params:", queryParams.value);
    if (isLoadingRef.current) {
      console.log("Already loading, skipping...");
      return;
    }
    
    isLoadingRef.current = true;
    try {
      loading.value = true;
      error.value = "";
      const response = await fetchTasks(queryParams.value);
      tasks.value = response.data;
      totalTasks.value = response.pagination.total_items || response.pagination.total || 0;
      totalPages.value = response.pagination.total_pages;
      currentPage.value = response.pagination.page;
    } catch (err) {
      console.error("Failed to load tasks:", err);
      error.value = "Failed to load tasks. Please try again.";
      // If paginated API fails, try legacy endpoint as fallback
      try {
        console.log("Trying fallback fetch...");
        const fallbackTasks = await fetchAllTasks();
        tasks.value = fallbackTasks;
        totalTasks.value = fallbackTasks.length;
        totalPages.value = 1;
      } catch (fallbackErr) {
        console.error("Fallback fetch also failed:", fallbackErr);
      }
    } finally {
      loading.value = false;
      isLoadingRef.current = false;
    }
  };

  // Load tasks on mount
  useEffect(() => {
    mounted.value = true;
    loadTasks();
  }, []); // Empty dependency array - only run once on mount

  // Track query parameter changes and reload tasks
  // Store previous params to detect actual changes
  const prevParamsRef = useRef<string>("");
  
  // Track query parameter changes and reload tasks
  effect(() => {
    if (mounted.value) {
      const paramsStr = JSON.stringify(queryParams.value);
      if (paramsStr !== prevParamsRef.current) {
        prevParamsRef.current = paramsStr;
        console.log("Query params changed:", queryParams.value);
        loadTasks();
      }
    }
  });

  // Cleanup search timeout on unmount
  useEffect(() => {
    return () => {
      if (searchTimeout) {
        clearTimeout(searchTimeout);
      }
    };
  }, []);

  const handleAddTask = async (title: string) => {
    if (title.trim() === "") {
      return;
    }

    try {
      error.value = "";
      const newTask = await createTask(title.trim());
      newTaskTitle.value = "";
      // Reload tasks to ensure consistency with backend state and pagination
      await loadTasks();
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
      // Reload tasks to ensure consistency with backend state and pagination
      await loadTasks();
    } catch (err) {
      console.error("Failed to delete task:", err);
      error.value = "Failed to delete task. Please try again.";
    }
  };

  // Search handlers with debouncing
  const handleSearchChange = (newSearchParams: SearchParams) => {
    searchParams.value = newSearchParams;
    
    // Clear existing timeout
    if (searchTimeout) {
      clearTimeout(searchTimeout);
    }
    
    // Debounce the search query, but apply status filter immediately
    if (newSearchParams.status !== queryParams.value.status) {
      // Status filter changes immediately
      queryParams.value = {
        ...queryParams.value,
        status: newSearchParams.status || "all",
        q: newSearchParams.q || "",
        page: 1 // Reset to first page
      };
    } else {
      // Debounce text search
      searchTimeout = setTimeout(() => {
        queryParams.value = {
          ...queryParams.value,
          q: newSearchParams.q || "",
          page: 1 // Reset to first page
        };
      }, 300); // 300ms debounce
    }
  };

  const handleClearSearch = () => {
    if (searchTimeout) {
      clearTimeout(searchTimeout);
    }
    
    searchParams.value = { q: "", status: "all" };
    queryParams.value = {
      ...queryParams.value,
      q: "",
      status: "all",
      page: 1
    };
  };

  // Pagination handlers
  const handleNextPage = () => {
    if (currentPage.value < totalPages.value) {
      queryParams.value = {
        ...queryParams.value,
        page: currentPage.value + 1
      };
    }
  };

  const handlePreviousPage = () => {
    if (currentPage.value > 1) {
      queryParams.value = {
        ...queryParams.value,
        page: currentPage.value - 1
      };
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

      {/* Search Form */}
      <div class="mt-8">
        <SearchForm
          searchParams={searchParams.value}
          onSearchChange={handleSearchChange}
          onClearSearch={handleClearSearch}
        />
      </div>

      {/* Task Results */}
      <div class="mt-6">
        {/* Results Summary */}
        {!loading.value && (
          <div class="mb-4 text-sm text-gray-600">
            {totalTasks.value > 0 
              ? `Showing ${tasks.value.length} of ${totalTasks.value} tasks`
              : searchParams.value.q || (searchParams.value.status && searchParams.value.status !== "all")
                ? "No tasks match your search criteria"
                : "No tasks yet"
            }
          </div>
        )}

        {loading.value
          ? (
            <div class="text-center py-8" data-testid="loading">
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
          
        {/* Pagination Controls */}
        {!loading.value && totalPages.value > 1 && (
          <div class="mt-6 flex justify-between items-center">
            <button
              onClick={handlePreviousPage}
              disabled={currentPage.value === 1}
              class={`px-4 py-2 rounded ${
                currentPage.value === 1
                  ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                  : "bg-blue-500 text-white hover:bg-blue-600"
              }`}
            >
              Back
            </button>
            
            <span class="text-gray-600">
              Page {currentPage.value} of {totalPages.value}
            </span>
            
            <button
              onClick={handleNextPage}
              disabled={currentPage.value === totalPages.value}
              class={`px-4 py-2 rounded ${
                currentPage.value === totalPages.value
                  ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                  : "bg-blue-500 text-white hover:bg-blue-600"
              }`}
            >
              Next
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
