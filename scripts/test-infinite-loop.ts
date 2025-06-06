#!/usr/bin/env -S deno run --allow-net --allow-read

// Test script to verify the infinite loop issue is fixed

const API_URL = "http://localhost:8001/api/tasks";
let requestCount = 0;
const startTime = Date.now();

// Mock server to count requests
const server = Deno.serve({ port: 8001 }, (req) => {
  const url = new URL(req.url);
  
  if (url.pathname === "/api/tasks") {
    requestCount++;
    console.log(`Request #${requestCount} at ${Date.now() - startTime}ms`);
    
    // Return mock response
    return new Response(JSON.stringify({
      data: [],
      pagination: {
        page: 1,
        page_size: 20,
        total_items: 0,
        total_pages: 0,
        has_next: false,
        has_previous: false
      }
    }), {
      headers: { "Content-Type": "application/json" }
    });
  }
  
  // For other paths, return 404
  return new Response("Not found", { status: 404 });
});

console.log("Test server running on http://localhost:8001");
console.log("Monitoring API requests for 5 seconds...");

// Run for 5 seconds then report
setTimeout(() => {
  console.log("\n--- Test Results ---");
  console.log(`Total requests in 5 seconds: ${requestCount}`);
  console.log(`Average requests per second: ${(requestCount / 5).toFixed(2)}`);
  
  if (requestCount > 10) {
    console.log("❌ FAIL: Too many requests detected. Possible infinite loop!");
  } else if (requestCount === 0) {
    console.log("⚠️  WARNING: No requests detected. Is the app running?");
  } else {
    console.log("✅ PASS: Request count is reasonable. No infinite loop detected.");
  }
  
  Deno.exit(0);
}, 5000);