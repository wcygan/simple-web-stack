/// <reference lib="dom" />
import { assertEquals, assertExists } from "@std/assert";
import { render, waitFor, fireEvent, cleanup } from "@testing-library/preact";
import { signal } from "@preact/signals";
import TodoApp from "../islands/TodoApp.tsx";
import "npm:global-jsdom@24.0.0/register";

// Mock the fetch API
let fetchCallCount = 0;
let fetchCallLog: string[] = [];

// Helper to create mock tasks
function createMockTask(id: string, title: string, completed: boolean = false) {
  return {
    id,
    title,
    completed,
    user_id: "user1",
    created_at: "2025-01-01T00:00:00Z",
    updated_at: "2025-01-01T00:00:00Z",
  };
}

// Create 25 mock tasks for pagination testing
const allMockTasks = Array.from({ length: 25 }, (_, i) => 
  createMockTask(`${i + 1}`, `Task ${i + 1}`, i % 3 === 0)
);

// Default mock response
const mockTasks = {
  data: allMockTasks.slice(0, 2),
  pagination: {
    page: 1,
    page_size: 20,
    total_items: 2,
    total_pages: 1,
    has_next: false,
    has_previous: false,
  },
};

// Store original fetch
const originalFetch = globalThis.fetch;

// Mock fetch globally
function setupFetchMock() {
  fetchCallCount = 0;
  fetchCallLog = [];
  
  globalThis.fetch = async (url: string | URL | Request, init?: RequestInit) => {
    const urlString = typeof url === 'string' ? url : url.toString();
    fetchCallCount++;
    fetchCallLog.push(`${init?.method || 'GET'} ${urlString}`);
    
    // Mock task list endpoint with pagination and search
    if (urlString.includes('/api/tasks') && (!init?.method || init.method === 'GET')) {
      const urlObj = new URL(urlString, 'http://localhost');
      const params = urlObj.searchParams;
      
      // Parse parameters
      const page = parseInt(params.get('page') || '1');
      const pageSize = parseInt(params.get('page_size') || '10');
      const searchQuery = params.get('q')?.toLowerCase() || '';
      const status = params.get('status') || 'all';
      
      // Filter tasks based on search and status
      let filteredTasks = [...allMockTasks];
      
      // Apply search filter
      if (searchQuery) {
        filteredTasks = filteredTasks.filter(task => 
          task.title.toLowerCase().includes(searchQuery)
        );
      }
      
      // Apply status filter
      if (status !== 'all') {
        if (status === 'completed') {
          filteredTasks = filteredTasks.filter(task => task.completed);
        } else if (status === 'pending') {
          filteredTasks = filteredTasks.filter(task => !task.completed);
        }
      }
      
      // Calculate pagination
      const totalItems = filteredTasks.length;
      const totalPages = Math.ceil(totalItems / pageSize);
      const startIndex = (page - 1) * pageSize;
      const endIndex = startIndex + pageSize;
      const paginatedTasks = filteredTasks.slice(startIndex, endIndex);
      
      const response = {
        data: paginatedTasks,
        pagination: {
          page,
          page_size: pageSize,
          total_items: totalItems,
          total_pages: totalPages,
          has_next: page < totalPages,
          has_previous: page > 1,
        },
      };
      
      return new Response(JSON.stringify(response), {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
      });
    }
    
    // Mock create task endpoint
    if (urlString.includes('/api/tasks') && init?.method === 'POST') {
      const body = JSON.parse(init.body as string);
      const newTask = {
        id: Date.now().toString(),
        title: body.title,
        completed: false,
        user_id: "user1",
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };
      allMockTasks.push(newTask);
      return new Response(JSON.stringify(newTask), {
        status: 201,
        headers: { 'Content-Type': 'application/json' },
      });
    }
    
    // Mock update task endpoint
    if (urlString.match(/\/api\/tasks\/\w+/) && init?.method === 'PUT') {
      const taskId = urlString.split('/').pop();
      const body = JSON.parse(init.body as string);
      const task = allMockTasks.find(t => t.id === taskId);
      if (task) {
        Object.assign(task, body);
        return new Response(JSON.stringify(task), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        });
      }
    }
    
    // Mock delete task endpoint
    if (urlString.match(/\/api\/tasks\/\w+/) && init?.method === 'DELETE') {
      const taskId = urlString.split('/').pop();
      const index = allMockTasks.findIndex(t => t.id === taskId);
      if (index > -1) {
        allMockTasks.splice(index, 1);
      }
      return new Response(null, { status: 204 });
    }
    
    return new Response('Not found', { status: 404 });
  };
}

// Restore original fetch
function teardownFetchMock() {
  globalThis.fetch = originalFetch;
}

Deno.test("TodoApp - should not create infinite loop on mount", async () => {
  setupFetchMock();
  
  try {
    const { container } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      const loadingElement = container.querySelector('[data-testid="loading"]');
      assertEquals(loadingElement, null, "Should not be loading after initial fetch");
    }, { timeout: 2000 });
    
    // Record initial fetch count
    const initialFetchCount = fetchCallCount;
    
    // Wait a bit to see if more fetches happen
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Check that fetch wasn't called excessively
    const additionalFetches = fetchCallCount - initialFetchCount;
    assertEquals(additionalFetches, 0, `Expected no additional fetches, but got ${additionalFetches}`);
    
    // Verify only one initial fetch was made
    assertEquals(fetchCallCount, 1, "Should only fetch tasks once on mount");
    assertEquals(fetchCallLog[0], "GET /api/tasks?page=1&page_size=10", "Should fetch tasks on mount with pagination params");
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should fetch when search parameters change", async () => {
  setupFetchMock();
  
  try {
    const { container, getByPlaceholderText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(container.querySelector('.bg-white'));
    });
    
    // Reset fetch count after initial load
    fetchCallCount = 0;
    fetchCallLog = [];
    
    // Type in search box
    const searchInput = getByPlaceholderText("Search by title...");
    fireEvent.input(searchInput, { target: { value: "test" } });
    
    // Wait for debounce (300ms) plus some buffer
    await new Promise(resolve => setTimeout(resolve, 400));
    
    // Should have made one fetch with search query
    assertEquals(fetchCallCount, 1, "Should fetch once after search");
    assertEquals(fetchCallLog[0].includes("q=test"), true, "Should include search query");
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should not create infinite loop when adding task", async () => {
  setupFetchMock();
  
  try {
    const { container, getByPlaceholderText, getByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(container.querySelector('.bg-white'));
    });
    
    // Reset fetch count
    fetchCallCount = 0;
    fetchCallLog = [];
    
    // Add a new task
    const input = getByPlaceholderText("What needs to be done?");
    fireEvent.input(input, { target: { value: "New test task" } });
    
    const addButton = getByText("Add Task");
    fireEvent.click(addButton);
    
    // Wait for task creation and reload
    await waitFor(() => {
      assertEquals(fetchCallLog.includes("POST /api/tasks"), true, "Should create task");
      assertEquals(fetchCallLog.includes("GET /api/tasks"), true, "Should reload tasks after creation");
    });
    
    // Wait to ensure no infinite loop
    await new Promise(resolve => setTimeout(resolve, 500));
    
    // Should only have 2 calls: POST to create, GET to reload
    assertEquals(fetchCallCount, 2, "Should only make 2 API calls when adding task");
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should handle task deletion without infinite loop", async () => {
  setupFetchMock();
  
  try {
    const { container, getAllByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(container.querySelector('.bg-white'));
    });
    
    // Reset fetch count
    fetchCallCount = 0;
    fetchCallLog = [];
    
    // Click delete on first task
    const deleteButtons = getAllByText("Delete");
    if (deleteButtons.length > 0) {
      fireEvent.click(deleteButtons[0]);
      
      // Wait for deletion and reload
      await waitFor(() => {
        assertEquals(fetchCallLog.some(log => log.includes("DELETE")), true, "Should delete task");
        assertEquals(fetchCallLog.includes("GET /api/tasks"), true, "Should reload after delete");
      });
      
      // Should only have 2 calls: DELETE and GET to reload
      assertEquals(fetchCallCount, 2, "Should only make 2 API calls when deleting");
    }
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should debounce search input", async () => {
  setupFetchMock();
  
  try {
    const { getByPlaceholderText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertEquals(fetchCallCount, 1, "Initial load");
    });
    
    // Reset fetch count
    fetchCallCount = 0;
    fetchCallLog = [];
    
    // Type multiple characters quickly
    const searchInput = getByPlaceholderText("Search by title...");
    fireEvent.input(searchInput, { target: { value: "t" } });
    fireEvent.input(searchInput, { target: { value: "te" } });
    fireEvent.input(searchInput, { target: { value: "tes" } });
    fireEvent.input(searchInput, { target: { value: "test" } });
    
    // Should not fetch immediately
    assertEquals(fetchCallCount, 0, "Should not fetch while typing");
    
    // Wait for debounce
    await new Promise(resolve => setTimeout(resolve, 400));
    
    // Should only fetch once after debounce
    assertEquals(fetchCallCount, 1, "Should fetch once after debounce");
    assertEquals(fetchCallLog[0].includes("q=test"), true, "Should search for final value");
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

// Run this test to specifically check for the infinite loop issue
Deno.test("TodoApp - stress test for infinite loops", async () => {
  setupFetchMock();
  
  try {
    const { container } = render(<TodoApp />);
    
    // Track fetch calls over time
    const fetchCounts: number[] = [];
    const checkInterval = setInterval(() => {
      fetchCounts.push(fetchCallCount);
    }, 100);
    
    // Run for 2 seconds
    await new Promise(resolve => setTimeout(resolve, 2000));
    clearInterval(checkInterval);
    
    // Analyze fetch pattern
    const totalFetches = fetchCallCount;
    const averageFetchesPerSecond = totalFetches / 2;
    
    console.log(`Total fetches in 2 seconds: ${totalFetches}`);
    console.log(`Average fetches per second: ${averageFetchesPerSecond}`);
    console.log(`Fetch log: ${fetchCallLog.join(", ")}`);
    
    // Should have very few fetches (just initial load)
    assertEquals(totalFetches <= 2, true, 
      `Too many fetches detected: ${totalFetches}. Possible infinite loop!`);
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

// Pagination Tests
Deno.test("TodoApp - should display only 10 tasks per page", async () => {
  setupFetchMock();
  
  try {
    // Wrap in error boundary to catch render errors
    try {
      const { container, queryByText, debug } = render(<TodoApp />);
      
      // Debug initial render
      console.log("Initial render HTML:");
      debug();
      
      // Add a small delay to ensure component mounts
      await new Promise(resolve => setTimeout(resolve, 100));
      
      // Wait for component to render
      await waitFor(() => {
        const wrapper = container.querySelector('.bg-white');
        console.log("Looking for .bg-white wrapper:", wrapper);
        assertExists(wrapper, "Component wrapper should exist");
      }, { timeout: 3000 });
      
      // Wait for tasks to load
      await waitFor(() => {
        const task1 = queryByText("Task 1");
        console.log("Looking for Task 1:", task1);
        assertExists(task1, "Task 1 should be visible");
      }, { timeout: 3000 });
      
      // Should show first 10 tasks (with 25 total tasks)
      await waitFor(() => {
        for (let i = 1; i <= 10; i++) {
          assertExists(queryByText(`Task ${i}`), `Task ${i} should be visible`);
        }
        // Task 11 should not be visible
        assertEquals(queryByText(`Task 11`), null, "Task 11 should not be visible on first page");
      });
      
      // Check that fetch was called with correct page_size
      const lastFetch = fetchCallLog[fetchCallLog.length - 1];
      assertEquals(lastFetch.includes('page_size=10'), true, "Should request 10 items per page");
      
    } catch (renderError) {
      console.error("Render error:", renderError);
      throw renderError;
    }
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should show Next and Back buttons for pagination", async () => {
  setupFetchMock();
  
  try {
    const { getByText, queryByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(getByText("Task 1"));
    });
    
    // Should show Next button on first page
    await waitFor(() => {
      assertExists(getByText("Next"), "Next button should exist on first page");
      assertEquals(queryByText("Back"), null, "Back button should not exist on first page");
    });
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should navigate to next page when Next is clicked", async () => {
  setupFetchMock();
  
  try {
    const { getByText, queryByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(getByText("Task 1"));
    });
    
    // Click Next button
    const nextButton = getByText("Next");
    fireEvent.click(nextButton);
    
    // Should show tasks 11-20
    await waitFor(() => {
      assertExists(getByText("Task 11"), "Task 11 should be visible on page 2");
      assertEquals(queryByText("Task 1"), null, "Task 1 should not be visible on page 2");
    });
    
    // Should show both Next and Back buttons on page 2
    await waitFor(() => {
      assertExists(getByText("Next"), "Next button should exist on page 2");
      assertExists(getByText("Back"), "Back button should exist on page 2");
    });
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should navigate back when Back is clicked", async () => {
  setupFetchMock();
  
  try {
    const { getByText, queryByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(getByText("Task 1"));
    });
    
    // Go to page 2
    const nextButton = getByText("Next");
    fireEvent.click(nextButton);
    
    await waitFor(() => {
      assertExists(getByText("Task 11"));
    });
    
    // Go back to page 1
    const backButton = getByText("Back");
    fireEvent.click(backButton);
    
    // Should show tasks 1-10 again
    await waitFor(() => {
      assertExists(getByText("Task 1"), "Task 1 should be visible again");
      assertEquals(queryByText("Task 11"), null, "Task 11 should not be visible");
    });
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

// Search Tests
Deno.test("TodoApp - should filter tasks by title when searching", async () => {
  setupFetchMock();
  
  try {
    const { getByPlaceholderText, queryByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(queryByText("Task 1"));
    });
    
    // Search for "Task 5"
    const searchInput = getByPlaceholderText("Search by title...");
    fireEvent.input(searchInput, { target: { value: "Task 5" } });
    
    // Wait for debounce
    await new Promise(resolve => setTimeout(resolve, 400));
    
    // Should only show Task 5, Task 15, and Task 25 (all contain "Task 5")
    await waitFor(() => {
      assertExists(queryByText("Task 5"), "Task 5 should be visible");
      assertEquals(queryByText("Task 1"), null, "Task 1 should not be visible");
      assertEquals(queryByText("Task 2"), null, "Task 2 should not be visible");
    });
    
    // Check the API was called with search query
    const searchFetch = fetchCallLog.find(log => log.includes("q=Task%205"));
    assertExists(searchFetch, "Should have made a search request with q=Task 5");
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should filter tasks by status", async () => {
  setupFetchMock();
  
  try {
    const { getByRole, queryByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(queryByText("Task 1"));
    });
    
    // Select "completed" status (tasks 3, 6, 9, 12, 15, 18, 21, 24)
    const statusSelect = getByRole("combobox") as HTMLSelectElement;
    fireEvent.change(statusSelect, { target: { value: "completed" } });
    
    // Should show only completed tasks
    await waitFor(() => {
      assertExists(queryByText("Task 3"), "Task 3 (completed) should be visible");
      assertEquals(queryByText("Task 1"), null, "Task 1 (not completed) should not be visible");
      assertEquals(queryByText("Task 2"), null, "Task 2 (not completed) should not be visible");
    });
    
    // Check the API was called with status filter
    const statusFetch = fetchCallLog.find(log => log.includes("status=completed"));
    assertExists(statusFetch, "Should have made a request with status=completed");
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should filter by both title and status simultaneously", async () => {
  setupFetchMock();
  
  try {
    const { getByPlaceholderText, getByRole, queryByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(queryByText("Task 1"));
    });
    
    // Select "completed" status
    const statusSelect = getByRole("combobox") as HTMLSelectElement;
    fireEvent.change(statusSelect, { target: { value: "completed" } });
    
    // Wait a bit
    await new Promise(resolve => setTimeout(resolve, 100));
    
    // Search for "Task 1"
    const searchInput = getByPlaceholderText("Search by title...");
    fireEvent.input(searchInput, { target: { value: "Task 1" } });
    
    // Wait for debounce
    await new Promise(resolve => setTimeout(resolve, 400));
    
    // Should show only completed tasks containing "Task 1" (Task 12, 15, 18)
    await waitFor(() => {
      assertExists(queryByText("Task 12"), "Task 12 (completed, contains 'Task 1') should be visible");
      assertExists(queryByText("Task 15"), "Task 15 (completed, contains 'Task 1') should be visible");
      assertExists(queryByText("Task 18"), "Task 18 (completed, contains 'Task 1') should be visible");
      assertEquals(queryByText("Task 1"), null, "Task 1 (not completed) should not be visible");
      assertEquals(queryByText("Task 3"), null, "Task 3 (completed but doesn't contain 'Task 1') should not be visible");
    });
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});

Deno.test("TodoApp - should reset to page 1 when searching", async () => {
  setupFetchMock();
  
  try {
    const { getByText, getByPlaceholderText, queryByText } = render(<TodoApp />);
    
    // Wait for initial load
    await waitFor(() => {
      assertExists(getByText("Task 1"));
    });
    
    // Go to page 2
    const nextButton = getByText("Next");
    fireEvent.click(nextButton);
    
    await waitFor(() => {
      assertExists(getByText("Task 11"));
    });
    
    // Perform a search
    const searchInput = getByPlaceholderText("Search by title...");
    fireEvent.input(searchInput, { target: { value: "Task" } });
    
    // Wait for debounce
    await new Promise(resolve => setTimeout(resolve, 400));
    
    // Should be back on page 1
    await waitFor(() => {
      assertExists(queryByText("Task 1"), "Should be back on page 1");
      assertEquals(queryByText("Task 11"), null, "Should not show page 2 items");
    });
    
    // Check that the request included page=1
    const searchFetch = fetchCallLog[fetchCallLog.length - 1];
    assertEquals(searchFetch.includes("page=1"), true, "Should reset to page 1 when searching");
    
  } finally {
    cleanup();
    teardownFetchMock();
  }
});