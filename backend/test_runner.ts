#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-env --allow-net

/**
 * Interactive test runner with timeout support for backend tests
 * 
 * This script runs both Rust unit tests and integration tests with proper timeout handling
 * and real-time progress indicators
 */

import { delay } from "https://deno.land/std@0.208.0/async/delay.ts";

interface TestResult {
  success: boolean;
  output: string;
  error?: string;
  timedOut: boolean;
  duration: number;
}

class ProgressIndicator {
  private interval: number | null = null;
  private frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
  private currentFrame = 0;

  start(message: string): void {
    Deno.stdout.writeSync(new TextEncoder().encode(`${message} `));
    this.interval = setInterval(() => {
      Deno.stdout.writeSync(new TextEncoder().encode(`\r${message} ${this.frames[this.currentFrame]}`));
      this.currentFrame = (this.currentFrame + 1) % this.frames.length;
    }, 100);
  }

  update(message: string): void {
    if (this.interval) {
      Deno.stdout.writeSync(new TextEncoder().encode(`\r${message} ${this.frames[this.currentFrame]}`));
    }
  }

  stop(finalMessage: string): void {
    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }
    Deno.stdout.writeSync(new TextEncoder().encode(`\r${finalMessage}\n`));
  }
}

class InteractiveTestRunner {
  private readonly cargoTestTimeout = 5000; // 5 seconds
  private readonly integrationTestTimeout = 30000; // 30 seconds
  private progress = new ProgressIndicator();

  async runCargoTests(): Promise<TestResult> {
    this.progress.start("🧪 Running Rust unit tests (5s timeout)");
    const startTime = Date.now();

    try {
      const command = new Deno.Command("cargo", {
        args: ["test", "--verbose"],
        stdout: "piped",
        stderr: "piped",
      });

      const process = command.spawn();
      
      // Update progress every 500ms
      const progressInterval = setInterval(() => {
        const elapsed = Math.floor((Date.now() - startTime) / 1000);
        this.progress.update(`🧪 Running Rust unit tests (${elapsed}s elapsed, 5s timeout)`);
      }, 500);

      // Create a timeout promise
      const timeoutPromise = delay(this.cargoTestTimeout).then(() => {
        clearInterval(progressInterval);
        process.kill("SIGTERM");
        throw new Error("TIMEOUT");
      });

      // Race between the process completion and timeout
      const result = await Promise.race([
        process.output(),
        timeoutPromise,
      ]);

      clearInterval(progressInterval);
      const duration = Date.now() - startTime;
      const output = new TextDecoder().decode(result.stdout);
      const error = new TextDecoder().decode(result.stderr);

      if (result.success) {
        this.progress.stop(`✅ Rust unit tests passed (${duration}ms)`);
        return {
          success: true,
          output,
          timedOut: false,
          duration,
        };
      } else {
        this.progress.stop(`❌ Rust unit tests failed (${duration}ms)`);
        console.log(`   Error output: ${error.slice(0, 200)}${error.length > 200 ? '...' : ''}`);
        return {
          success: false,
          output,
          error,
          timedOut: false,
          duration,
        };
      }
    } catch (error) {
      const duration = Date.now() - startTime;
      
      if (error instanceof Error && error.message === "TIMEOUT") {
        this.progress.stop(`❌ Rust unit tests timed out after 5 seconds`);
        return {
          success: false,
          output: "",
          error: "Tests timed out after 5 seconds",
          timedOut: true,
          duration,
        };
      }

      this.progress.stop(`❌ Rust unit tests failed with error`);
      console.log(`   Error: ${error instanceof Error ? error.message : String(error)}`);
      return {
        success: false,
        output: "",
        error: error instanceof Error ? error.message : String(error),
        timedOut: false,
        duration,
      };
    }
  }

  async runIntegrationTests(): Promise<TestResult> {
    this.progress.start("🔗 Starting integration tests");
    const startTime = Date.now();
    let process: Deno.ChildProcess | null = null;
    const abortController = new AbortController();

    try {
      const command = new Deno.Command("./integration_test.ts", {
        stdout: "piped",
        stderr: "piped",
      });

      process = command.spawn();
      
      // Update progress every 1000ms for integration tests
      const progressInterval = setInterval(() => {
        const elapsed = Math.floor((Date.now() - startTime) / 1000);
        this.progress.update(`🔗 Running integration tests (${elapsed}s elapsed, 30s timeout)`);
      }, 1000);

      // Create a timeout promise that can be cancelled
      const timeoutPromise = new Promise<never>((_, reject) => {
        const timeoutId = setTimeout(async () => {
          if (!abortController.signal.aborted) {
            clearInterval(progressInterval);
            if (process) {
              process.kill("SIGTERM");
              // Give it a moment, then force kill
              await delay(1000);
              process.kill("SIGKILL");
            }
            reject(new Error("TIMEOUT"));
          }
        }, this.integrationTestTimeout);

        // Cancel timeout if aborted
        abortController.signal.addEventListener('abort', () => {
          clearTimeout(timeoutId);
        });
      });

      // Race between the process completion and timeout
      const result = await Promise.race([
        process.output(),
        timeoutPromise,
      ]);

      // Cancel the timeout since we completed successfully
      abortController.abort();
      clearInterval(progressInterval);
      
      const duration = Date.now() - startTime;
      const output = new TextDecoder().decode(result.stdout);
      const error = new TextDecoder().decode(result.stderr);

      if (result.success) {
        this.progress.stop(`✅ Integration tests passed (${duration}ms)`);
        // Show a summary of what was tested
        const lines = output.split('\n');
        const testLines = lines.filter(line => line.includes('✅') || line.includes('❌'));
        if (testLines.length > 0) {
          console.log(`   Completed ${testLines.length} integration test scenarios`);
        }
        return {
          success: true,
          output,
          timedOut: false,
          duration,
        };
      } else {
        this.progress.stop(`❌ Integration tests failed (${duration}ms)`);
        console.log(`   Error output: ${error.slice(0, 200)}${error.length > 200 ? '...' : ''}`);
        return {
          success: false,
          output,
          error,
          timedOut: false,
          duration,
        };
      }
    } catch (error) {
      // Cancel the timeout
      abortController.abort();
      
      const duration = Date.now() - startTime;
      
      // Ensure process is cleaned up
      if (process) {
        try {
          process.kill("SIGKILL");
        } catch {
          // Ignore errors during cleanup
        }
      }
      
      if (error instanceof Error && error.message === "TIMEOUT") {
        this.progress.stop(`❌ Integration tests timed out after 30 seconds`);
        return {
          success: false,
          output: "",
          error: "Integration tests timed out after 30 seconds",
          timedOut: true,
          duration,
        };
      }

      this.progress.stop(`❌ Integration tests failed with error`);
      console.log(`   Error: ${error instanceof Error ? error.message : String(error)}`);
      return {
        success: false,
        output: "",
        error: error instanceof Error ? error.message : String(error),
        timedOut: false,
        duration,
      };
    }
  }

  async runAllTests(): Promise<void> {
    console.log("🚀 Starting backend test suite with timeout protection");
    console.log("=".repeat(60));
    console.log("📋 Test Plan:");
    console.log("   1. 🦀 Rust unit tests (5s timeout)");
    console.log("   2. 🔗 Integration tests (30s timeout)");
    console.log("=".repeat(60));

    // Show current status
    console.log("\n📊 Progress:");
    console.log("   🦀 Rust unit tests: ⏳ Pending");
    console.log("   🔗 Integration tests: ⏳ Pending");
    console.log("");

    // Run cargo tests first
    const cargoResult = await this.runCargoTests();
    
    // Update status after cargo tests
    console.log("\n📊 Progress Update:");
    console.log(`   🦀 Rust unit tests: ${cargoResult.success ? '✅ Completed' : '❌ Failed'}`);
    console.log("   🔗 Integration tests: ⏳ Starting...");
    console.log("");
    
    // Always run integration tests, even if cargo tests fail
    const integrationResult = await this.runIntegrationTests();

    // Final status update
    console.log("\n📊 Final Status:");
    console.log(`   🦀 Rust unit tests: ${cargoResult.success ? '✅ Completed' : '❌ Failed'}`);
    console.log(`   🔗 Integration tests: ${integrationResult.success ? '✅ Completed' : '❌ Failed'}`);

    // Print detailed summary
    this.printDetailedSummary(cargoResult, integrationResult);

    // Exit with appropriate code
    if (!cargoResult.success || !integrationResult.success) {
      Deno.exit(1);
    }
  }

  private printDetailedSummary(cargoResult: TestResult, integrationResult: TestResult): void {
    console.log("\n" + "=".repeat(60));
    console.log("📊 DETAILED TEST SUMMARY");
    console.log("=".repeat(60));

    // Performance metrics
    const totalDuration = cargoResult.duration + integrationResult.duration;
    const allPassed = cargoResult.success && integrationResult.success;

    console.log(`\n⏱️  Performance Metrics:`);
    console.log(`   Total execution time: ${totalDuration}ms (${(totalDuration/1000).toFixed(1)}s)`);
    console.log(`   Rust compilation + tests: ${cargoResult.duration}ms`);
    console.log(`   Integration test suite: ${integrationResult.duration}ms`);

    console.log(`\n🦀 Rust Unit Tests:`);
    console.log(`   Status: ${cargoResult.success ? "✅ PASSED" : "❌ FAILED"}`);
    console.log(`   Duration: ${cargoResult.duration}ms`);
    if (cargoResult.timedOut) {
      console.log(`   ⏰ TIMED OUT after 5 seconds`);
      console.log(`   💡 Tip: Check for infinite loops or blocking operations in tests`);
    }
    if (cargoResult.error) {
      console.log(`   Error: ${cargoResult.error}`);
    }

    console.log(`\n🔗 Integration Tests:`);
    console.log(`   Status: ${integrationResult.success ? "✅ PASSED" : "❌ FAILED"}`);
    console.log(`   Duration: ${integrationResult.duration}ms`);
    if (integrationResult.timedOut) {
      console.log(`   ⏰ TIMED OUT after 30 seconds`);
      console.log(`   💡 Tip: Check server startup or network connectivity issues`);
    }
    if (integrationResult.error) {
      console.log(`   Error: ${integrationResult.error}`);
    }

    // Overall result with recommendations
    console.log(`\n🎯 Overall Result: ${allPassed ? "✅ ALL TESTS PASSED" : "❌ SOME TESTS FAILED"}`);
    
    if (allPassed) {
      console.log(`🎉 Great job! All tests completed successfully in ${(totalDuration/1000).toFixed(1)}s`);
      if (totalDuration > 10000) {
        console.log(`💡 Tip: Tests took ${(totalDuration/1000).toFixed(1)}s. Consider optimizing for faster feedback.`);
      }
    } else {
      console.log(`🔍 Check the error details above to fix failing tests`);
      if (cargoResult.timedOut || integrationResult.timedOut) {
        console.log(`⚠️  Timeout detected - this may indicate hanging tests or slow compilation`);
      }
    }

    console.log("=".repeat(60));
  }
}

// Main execution
async function main(): Promise<void> {
  const runner = new InteractiveTestRunner();
  await runner.runAllTests();
}

if (import.meta.main) {
  await main();
} 