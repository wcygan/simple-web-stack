#!/usr/bin/env -S deno run --allow-read --allow-run

/**
 * Test Runner for Simple Web Stack Backend
 * Runs both unit tests (Rust) and integration tests (Deno)
 */

import { delay } from "https://deno.land/std@0.208.0/async/delay.ts";

// Colors for output
const colors = {
  green: "\x1b[32m",
  red: "\x1b[31m", 
  blue: "\x1b[34m",
  reset: "\x1b[0m"
};

function printStatus(message: string): void {
  console.log(`${colors.blue}${message}${colors.reset}`);
}

function printSuccess(message: string): void {
  console.log(`${colors.green}✅ ${message}${colors.reset}`);
}

function printError(message: string): void {
  console.log(`${colors.red}❌ ${message}${colors.reset}`);
}

async function checkCommand(command: string): Promise<boolean> {
  try {
    const cmd = new Deno.Command(command, {
      args: ["--version"],
      stdout: "null",
      stderr: "null"
    });
    const result = await cmd.output();
    return result.success;
  } catch {
    return false;
  }
}

async function runCommand(command: string, args: string[], cwd?: string): Promise<boolean> {
  try {
    const cmd = new Deno.Command(command, {
      args,
      cwd,
      stdout: "inherit",
      stderr: "inherit"
    });
    const result = await cmd.output();
    return result.success;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    printError(`Failed to run command: ${errorMessage}`);
    return false;
  }
}

async function main(): Promise<void> {
  console.log("🧪 Simple Web Stack Backend Test Suite");
  console.log("======================================");

  // Check prerequisites
  printStatus("\n🔍 Checking prerequisites...");

  const cargoAvailable = await checkCommand("cargo");
  if (!cargoAvailable) {
    printError("Cargo not found. Please install Rust.");
    Deno.exit(1);
  }

  const denoAvailable = await checkCommand("deno");
  if (!denoAvailable) {
    printError("Deno not found. Please install Deno.");
    Deno.exit(1);
  }

  printSuccess("Prerequisites check passed");

  // Run unit tests
  printStatus("\n🦀 Running Rust unit tests...");
  const unitTestsSuccess = await runCommand("cargo", ["test"]);
  
  if (!unitTestsSuccess) {
    printError("Unit tests failed");
    Deno.exit(1);
  }
  
  printSuccess("Unit tests passed");

  // Run integration tests
  printStatus("\n🦕 Running Deno integration tests...");
  const integrationTestsSuccess = await runCommand("./integration_test.ts", []);
  
  if (!integrationTestsSuccess) {
    printError("Integration tests failed");
    Deno.exit(1);
  }
  
  printSuccess("Integration tests passed");

  printStatus("\n🎉 All tests completed successfully!");
  console.log("======================================");
}

if (import.meta.main) {
  main();
} 