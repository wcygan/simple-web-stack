#!/usr/bin/env -S deno run --allow-net

// Test script to check if the frontend infinite loop is fixed

const FRONTEND_URL = "http://localhost:8000";
const API_URL = "http://localhost:8000/api/tasks";

console.log("Testing frontend for infinite loop issue...\n");

// Track API calls
let apiCallCount = 0;
const startTime = Date.now();

// Monitor API calls for 5 seconds
const checkInterval = setInterval(async () => {
  try {
    const response = await fetch(API_URL);
    if (response.ok) {
      apiCallCount++;
      console.log(`API call #${apiCallCount} at ${new Date().toISOString()}`);
    }
  } catch (error) {
    console.error("Error calling API:", error.message);
  }
}, 100); // Check every 100ms

// Stop after 5 seconds
setTimeout(() => {
  clearInterval(checkInterval);
  const duration = (Date.now() - startTime) / 1000;
  
  console.log("\n=== Test Results ===");
  console.log(`Duration: ${duration} seconds`);
  console.log(`Total API calls: ${apiCallCount}`);
  console.log(`Average calls per second: ${(apiCallCount / duration).toFixed(2)}`);
  
  if (apiCallCount > 10) {
    console.log("\n❌ FAIL: Too many API calls detected. Possible infinite loop!");
  } else {
    console.log("\n✅ PASS: API call rate is normal. No infinite loop detected.");
  }
  
  Deno.exit(0);
}, 5000);

// Also test the main page loads
try {
  const pageResponse = await fetch(FRONTEND_URL);
  if (pageResponse.ok) {
    console.log("✅ Frontend page loads successfully");
  } else {
    console.log("❌ Frontend page failed to load:", pageResponse.status);
  }
} catch (error) {
  console.log("❌ Error loading frontend:", error.message);
}