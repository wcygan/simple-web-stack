#!/usr/bin/env -S deno run --allow-net

// Manual test script for pagination and search functionality

const API_BASE = "http://localhost:3000";

async function testPagination() {
  console.log("=== Testing Pagination ===\n");
  
  // Test page 1 with 10 items
  console.log("Test 1: Fetching page 1 with 10 items per page");
  const page1 = await fetch(`${API_BASE}/tasks?page=1&page_size=10`);
  const page1Data = await page1.json();
  console.log(`- Total items: ${page1Data.pagination.total_items}`);
  console.log(`- Total pages: ${page1Data.pagination.total_pages}`);
  console.log(`- Items on page 1: ${page1Data.data.length}`);
  console.log(`- Has next: ${page1Data.pagination.has_next}`);
  console.log(`- Has previous: ${page1Data.pagination.has_previous}`);
  
  // Test page 2
  if (page1Data.pagination.has_next) {
    console.log("\nTest 2: Fetching page 2");
    const page2 = await fetch(`${API_BASE}/tasks?page=2&page_size=10`);
    const page2Data = await page2.json();
    console.log(`- Items on page 2: ${page2Data.data.length}`);
    console.log(`- Has next: ${page2Data.pagination.has_next}`);
    console.log(`- Has previous: ${page2Data.pagination.has_previous}`);
  }
}

async function testSearch() {
  console.log("\n\n=== Testing Search ===\n");
  
  // Test search by title
  console.log("Test 1: Search by title (q=task)");
  const searchTitle = await fetch(`${API_BASE}/tasks?q=task&page_size=10`);
  const searchTitleData = await searchTitle.json();
  console.log(`- Found ${searchTitleData.pagination.total_items} tasks matching 'task'`);
  console.log(`- First few titles:`, searchTitleData.data.slice(0, 3).map(t => t.title));
  
  // Test filter by status
  console.log("\nTest 2: Filter by status (status=completed)");
  const filterCompleted = await fetch(`${API_BASE}/tasks?status=completed&page_size=10`);
  const filterCompletedData = await filterCompleted.json();
  console.log(`- Found ${filterCompletedData.pagination.total_items} completed tasks`);
  console.log(`- All completed:`, filterCompletedData.data.every(t => t.completed));
  
  console.log("\nTest 3: Filter by status (status=pending)");
  const filterPending = await fetch(`${API_BASE}/tasks?status=pending&page_size=10`);
  const filterPendingData = await filterPending.json();
  console.log(`- Found ${filterPendingData.pagination.total_items} pending tasks`);
  console.log(`- All pending:`, filterPendingData.data.every(t => !t.completed));
  
  // Test combined search and filter
  console.log("\nTest 4: Combined search and filter (q=task&status=completed)");
  const combined = await fetch(`${API_BASE}/tasks?q=task&status=completed&page_size=10`);
  const combinedData = await combined.json();
  console.log(`- Found ${combinedData.pagination.total_items} completed tasks matching 'task'`);
  console.log(`- Sample:`, combinedData.data.slice(0, 2).map(t => ({title: t.title, completed: t.completed})));
}

async function createTestData() {
  console.log("=== Creating Test Data ===\n");
  
  // Create 25 test tasks
  for (let i = 1; i <= 25; i++) {
    const title = `Test Task ${i}`;
    const response = await fetch(`${API_BASE}/tasks`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ title }),
    });
    
    if (response.ok) {
      const task = await response.json();
      console.log(`Created: ${task.title}`);
      
      // Mark every 3rd task as completed
      if (i % 3 === 0) {
        await fetch(`${API_BASE}/tasks/${task.id}`, {
          method: "PUT",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ completed: true }),
        });
        console.log(`  -> Marked as completed`);
      }
    }
  }
}

// Main execution
async function main() {
  console.log("Waiting for backend to be ready...");
  
  // Wait for backend
  let attempts = 0;
  while (attempts < 30) {
    try {
      const response = await fetch(`http://localhost:3000/health`);
      if (response.ok) {
        console.log("Backend is ready!\n");
        break;
      }
    } catch {
      // Continue waiting
    }
    attempts++;
    await new Promise(resolve => setTimeout(resolve, 1000));
  }
  
  if (attempts >= 30) {
    console.error("Backend did not start in time");
    Deno.exit(1);
  }
  
  // Check if we need test data
  const tasksResponse = await fetch(`${API_BASE}/tasks`);
  const tasksData = await tasksResponse.json();
  
  if (tasksData.length < 20) {
    console.log("Not enough test data, creating some...\n");
    await createTestData();
    console.log("\n");
  }
  
  // Run tests
  await testPagination();
  await testSearch();
  
  console.log("\n✅ All tests completed!");
}

main().catch(console.error);