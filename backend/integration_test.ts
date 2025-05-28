#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-net

/**
 * Integration tests for the Simple Web Stack Backend
 * 
 * This script tests the actual HTTP server by:
 * 1. Starting the Rust server as a subprocess (optimized for speed)
 * 2. Waiting for it to be ready
 * 3. Running HTTP tests against the live server
 * 4. Cleaning up the server process
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

  async start(): Promise<void> {
    console.log("🚀 Starting Rust server (debug mode for speed)...");
    
    // Start the server directly without separate build step
    // Use debug mode for faster compilation
    const command = new Deno.Command("cargo", {
      args: ["run"], // Removed --release flag for faster builds
      stdout: "piped",
      stderr: "piped",
    });

    this.process = command.spawn();

    // Wait for server to be ready
    await this.waitForServer();
    console.log("✅ Server is ready!");
  }

  async stop(): Promise<void> {
    if (this.process) {
      console.log("🛑 Stopping server...");
      
      // Try graceful shutdown first
      this.process.kill("SIGTERM");
      
      // Wait for graceful shutdown with timeout
      const shutdownTimeout = 2000; // 2 seconds
      const shutdownPromise = this.process.status;
      const timeoutPromise = delay(shutdownTimeout).then(() => {
        console.log("⚠️  Graceful shutdown timed out, forcing termination...");
        this.process?.kill("SIGKILL");
        return this.process?.status;
      });
      
      try {
        await Promise.race([shutdownPromise, timeoutPromise]);
      } catch (error) {
        console.log(`⚠️  Error during shutdown: ${error}`);
        // Force kill if there's an error
        this.process?.kill("SIGKILL");
      }
      
      this.process = null;
      console.log("✅ Server stopped");
    }
  }

  private async waitForServer(): Promise<void> {
    const startTime = Date.now();
    
    while (Date.now() - startTime < this.maxStartupTime) {
      try {
        const response = await fetch(`${this.serverUrl}/health`, {
          signal: AbortSignal.timeout(500), // Reduced timeout for faster failure detection
        });
        
        if (response.ok) {
          return; // Server is ready
        }
      } catch {
        // Server not ready yet, continue waiting
      }
      
      await delay(this.healthCheckInterval);
    }
    
    throw new Error(`Server failed to start within ${this.maxStartupTime}ms`);
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
  await tester.runAllTests();
}

if (import.meta.main) {
  await main();
} 