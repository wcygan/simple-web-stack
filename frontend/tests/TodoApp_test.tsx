import { assertEquals, assertExists } from "@std/assert";
import { render, waitFor, fireEvent, cleanup } from "@testing-library/preact";
import { signal } from "@preact/signals";
import TodoApp from "../islands/TodoApp.tsx";

// Mock the fetch API
let fetchCallCount = 0;
let fetchCallLog: string[] = [];

const mockTasks = {
  data: [
    {
      id: "1",
      title: "Test Task 1",
      completed: false,
      user_id: "user1",
      created_at: "2025-01-01T00:00:00Z",
      updated_at: "2025-01-01T00:00:00Z",
    },
    {
      id: "2",
      title: "Test Task 2",
      completed: true,
      user_id: "user1",
      created_at: "2025-01-02T00:00:00Z",
      updated_at: "2025-01-02T00:00:00Z",
    },
  ],
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
    
    // Mock task list endpoint
    if (urlString.includes('/api/tasks') && (!init?.method || init.method === 'GET')) {
      return new Response(JSON.stringify(mockTasks), {
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
      return new Response(JSON.stringify(newTask), {
        status: 201,
        headers: { 'Content-Type': 'application/json' },
      });
    }
    
    // Mock update task endpoint
    if (urlString.match(/\/api\/tasks\/\w+/) && init?.method === 'PUT') {
      const taskId = urlString.split('/').pop();
      const body = JSON.parse(init.body as string);
      const task = mockTasks.data.find(t => t.id === taskId);
      if (task) {
        const updatedTask = { ...task, ...body };
        return new Response(JSON.stringify(updatedTask), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        });
      }
    }
    
    // Mock delete task endpoint
    if (urlString.match(/\/api\/tasks\/\w+/) && init?.method === 'DELETE') {
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
    assertEquals(fetchCallLog[0], "GET /api/tasks", "Should fetch tasks on mount");
    
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