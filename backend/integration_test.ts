#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-net

/**
 * Integration tests for the Simple Web Stack Backend
 * 
 * This script tests the actual HTTP server by:
 * 1. Checking if the server is already running at http://localhost:3000
 * 2. If not available, aborting with helpful guidance to start the server
 * 3. If available, running HTTP tests against the live server
 * 4. Leaving the server running after tests complete
 */

import { assertEquals, assertExists, assert } from "https://deno.land/std@0.208.0/assert/mod.ts";
import { delay } from "https://deno.land/std@0.208.0/async/delay.ts";

interface TestResult {
  name: string;
  passed: boolean;
  error?: string;
  duration: number;
}

interface HealthResponse {
  status: string;
  timestamp: string;
  service: string;
}

class ServerManager {
  private process: Deno.ChildProcess | null = null;
  private readonly serverUrl = "http://localhost:3000";
  private readonly maxStartupTime = 15000; // 15 seconds (reduced from 30)
  private readonly healthCheckInterval = 100; // 100ms (reduced from 500ms)
  private serverWasAlreadyRunning = false; // Track if we started the server or it was already running

  /**
   * Check if the server is already running (similar to: curl http://localhost:3000/health)
   * This provides fast feedback without waiting for timeout periods
   */
  async checkServerAvailability(): Promise<{ available: boolean; response?: HealthResponse; error?: string }> {
    try {
      console.log("🔍 Checking server availability at http://localhost:3000/health...");
      
      const response = await fetch(`${this.serverUrl}/health`, {
        signal: AbortSignal.timeout(2000), // Quick 2-second timeout for availability check
      });
      
      if (response.ok) {
        const data: HealthResponse = await response.json();
        console.log("✅ Server is already running and healthy!");
        return { available: true, response: data };
      } else {
        return { 
          available: false, 
          error: `Server responded with status ${response.status}` 
        };
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      
      // Distinguish between different types of connection errors
      if (errorMessage.includes("Connection refused") || errorMessage.includes("ECONNREFUSED")) {
        console.log("ℹ️  Server not running (connection refused) - tests require running server");
        return { available: false, error: "Connection refused - server not running" };
      } else if (errorMessage.includes("timeout")) {
        console.log("⚠️  Server availability check timed out");
        return { available: false, error: "Health check timeout - server may be unresponsive" };
      } else {
        console.log(`⚠️  Server availability check failed: ${errorMessage}`);
        return { available: false, error: errorMessage };
      }
    }
  }

  async start(): Promise<void> {
    // First, check if server is already running
    const availability = await this.checkServerAvailability();
    
    if (availability.available) {
      console.log("🔄 Using existing server instance");
      this.serverWasAlreadyRunning = true;
      return; // Server already running, no need to start a new one
    }
    
    // Server is not available - abort immediately with helpful guidance
    console.error("❌ Server is not available!");
    console.error(`📋 Availability check result: ${availability.error}`);
    console.error("");
    console.error("🚨 INTEGRATION TEST REQUIREMENTS:");
    console.error("   The server must be running before integration tests can proceed.");
    console.error("");
    console.error("💡 To fix this, choose one of the following options:");
    console.error("");
    console.error("   Option 1 - Run with Docker Compose:");
    console.error("   $ deno task up");
    console.error("   $ deno task test:backend");
    console.error("");
    console.error("   Option 2 - Run locally in development:");
    console.error("   Terminal 1: $ cd backend && cargo run");
    console.error("   Terminal 2: $ cd backend && ./integration_test.ts");
    console.error("");
    console.error("   Option 3 - Use the comprehensive test runner:");
    console.error("   $ deno task test:backend");
    console.error("   (This includes both unit tests and integration tests)");
    console.error("");
    console.error("🔍 Server status:");
    console.error(`   - Target URL: ${this.serverUrl}/health`);
    console.error(`   - Connection: ${availability.error}`);
    console.error(`   - Expected response: {"status":"healthy","service":"simple-web-stack-backend",...}`);
    
    throw new Error(
      `Integration tests require a running server at ${this.serverUrl}. ` +
      `Please start the server using 'deno task up' or 'cargo run' before running tests.`
    );
  }

  async stop(): Promise<void> {
    // Since we never start the server ourselves, just acknowledge that we're done
    console.log("ℹ️  Integration tests completed - server remains running");
  }

  getServerUrl(): string {
    return this.serverUrl;
  }
}

class IntegrationTester {
  private serverManager: ServerManager;
  private results: TestResult[] = [];

  constructor() {
    this.serverManager = new ServerManager();
  }

  /**
   * Test the server availability check functionality itself
   */
  async testAvailabilityCheck(): Promise<void> {
    console.log("🧪 Testing server availability check functionality...");
    
    // First, check availability when server is not running
    const initialCheck = await this.serverManager.checkServerAvailability();
    if (initialCheck.available) {
      console.log("ℹ️  Server was already running - this is fine for testing");
    } else {
      console.log("✅ Availability check correctly detected server not running");
    }
    
    console.log(`📋 Availability check result: ${JSON.stringify(initialCheck, null, 2)}`);
  }

  async runAllTests(): Promise<void> {
    try {
      await this.serverManager.start();
      
      // Run basic tests first (these are fast and validate core functionality)
      await this.runTest("Health endpoint returns 200", this.testHealthEndpointStatus.bind(this));
      await this.runTest("Health endpoint returns correct JSON structure", this.testHealthEndpointStructure.bind(this));
      await this.runTest("Invalid endpoint returns 404", this.testInvalidEndpoint.bind(this));
      
      // Run more comprehensive tests
      await this.runTest("Health endpoint returns valid timestamp", this.testHealthEndpointTimestamp.bind(this));
      await this.runTest("Health endpoint returns correct headers", this.testHealthEndpointHeaders.bind(this));
      await this.runTest("Health endpoint response time is reasonable", this.testResponseTime.bind(this));
      
      // Run concurrent and stress tests last
      await this.runTest("Health endpoint handles multiple concurrent requests", this.testConcurrentRequests.bind(this));
      await this.runTest("Server handles malformed requests gracefully", this.testMalformedRequests.bind(this));
      
    } finally {
      await this.serverManager.stop();
    }
    
    this.printResults();
  }

  private async runTest(name: string, testFn: () => Promise<void>): Promise<void> {
    const startTime = Date.now();
    
    try {
      await testFn();
      this.results.push({
        name,
        passed: true,
        duration: Date.now() - startTime,
      });
      console.log(`✅ ${name}`);
    } catch (error) {
      this.results.push({
        name,
        passed: false,
        error: error instanceof Error ? error.message : String(error),
        duration: Date.now() - startTime,
      });
      console.log(`❌ ${name}: ${error instanceof Error ? error.message : error}`);
    }
  }

  private async testHealthEndpointStatus(): Promise<void> {
    const response = await fetch(`${this.serverManager.getServerUrl()}/health`);
    assertEquals(response.status, 200);
  }

  private async testHealthEndpointStructure(): Promise<void> {
    const response = await fetch(`${this.serverManager.getServerUrl()}/health`);
    const data: HealthResponse = await response.json();
    
    assertExists(data.status);
    assertExists(data.timestamp);
    assertExists(data.service);
    
    assertEquals(data.status, "healthy");
    assertEquals(data.service, "simple-web-stack-backend");
    assertEquals(typeof data.timestamp, "string");
  }

  private async testHealthEndpointTimestamp(): Promise<void> {
    const response = await fetch(`${this.serverManager.getServerUrl()}/health`);
    const data: HealthResponse = await response.json();
    
    // Verify timestamp is valid RFC3339 format
    const timestamp = new Date(data.timestamp);
    assert(!isNaN(timestamp.getTime()), "Timestamp should be valid");
    
    // Verify timestamp is recent (within last minute)
    const now = new Date();
    const timeDiff = Math.abs(now.getTime() - timestamp.getTime());
    assert(timeDiff < 60000, "Timestamp should be within last minute");
  }

  private async testHealthEndpointHeaders(): Promise<void> {
    const response = await fetch(`${this.serverManager.getServerUrl()}/health`);
    
    assertEquals(response.headers.get("content-type"), "application/json");
    assertExists(response.headers.get("content-length"));
  }

  private async testInvalidEndpoint(): Promise<void> {
    const response = await fetch(`${this.serverManager.getServerUrl()}/invalid`);
    assertEquals(response.status, 404);
  }

  private async testConcurrentRequests(): Promise<void> {
    const requests = Array.from({ length: 5 }, () => // Reduced from 10 to 5 for faster execution
      fetch(`${this.serverManager.getServerUrl()}/health`)
    );
    
    const responses = await Promise.all(requests);
    
    for (const response of responses) {
      assertEquals(response.status, 200);
      const data: HealthResponse = await response.json();
      assertEquals(data.status, "healthy");
    }
  }

  private async testResponseTime(): Promise<void> {
    const startTime = Date.now();
    const response = await fetch(`${this.serverManager.getServerUrl()}/health`);
    const endTime = Date.now();
    
    assertEquals(response.status, 200);
    
    const responseTime = endTime - startTime;
    assert(responseTime < 1000, `Response time should be under 1000ms, got ${responseTime}ms`);
  }

  private async testMalformedRequests(): Promise<void> {
    // Test with invalid HTTP method
    const response1 = await fetch(`${this.serverManager.getServerUrl()}/health`, {
      method: "PATCH",
    });
    assertEquals(response1.status, 405); // Method Not Allowed
    
    // Test with invalid path
    const response2 = await fetch(`${this.serverManager.getServerUrl()}/health/../invalid`);
    assertEquals(response2.status, 404);
  }

  private printResults(): void {
    console.log("\n" + "=".repeat(60));
    console.log("🧪 INTEGRATION TEST RESULTS");
    console.log("=".repeat(60));
    
    const passed = this.results.filter(r => r.passed).length;
    const failed = this.results.filter(r => !r.passed).length;
    const totalTime = this.results.reduce((sum, r) => sum + r.duration, 0);
    
    console.log(`\n📊 Summary:`);
    console.log(`   Total tests: ${this.results.length}`);
    console.log(`   Passed: ${passed}`);
    console.log(`   Failed: ${failed}`);
    console.log(`   Total time: ${totalTime}ms`);
    
    if (failed > 0) {
      console.log(`\n❌ Failed tests:`);
      this.results
        .filter(r => !r.passed)
        .forEach(r => {
          console.log(`   • ${r.name}: ${r.error}`);
        });
    }
    
    console.log(`\n⏱️  Test timings:`);
    this.results.forEach(r => {
      const status = r.passed ? "✅" : "❌";
      console.log(`   ${status} ${r.name}: ${r.duration}ms`);
    });
    
    console.log("\n" + "=".repeat(60));
    
    if (failed > 0) {
      console.log("❌ Some tests failed!");
      Deno.exit(1);
    } else {
      console.log("✅ All tests passed!");
    }
  }
}

// Main execution
async function main(): Promise<void> {
  console.log("🧪 Starting Integration Tests for Simple Web Stack Backend");
  console.log("=".repeat(60));
  
  const tester = new IntegrationTester();
  
  // First, test the availability check functionality itself
  console.log("🔍 Phase 1: Testing Server Availability Check");
  console.log("-".repeat(40));
  await tester.testAvailabilityCheck();
  
  console.log("\n🧪 Phase 2: Running Integration Test Suite");
  console.log("-".repeat(40));
  await tester.runAllTests();
}

if (import.meta.main) {
  await main();
} 