#!/usr/bin/env -S deno run --allow-all
import $ from "jsr:@david/dax";
import { parseArgs } from "@std/cli/parse-args";
import { cyan, yellow, green, red, blue, magenta } from "@std/fmt/colors";

const args = parseArgs(Deno.args, {
  boolean: ["unit", "integration", "performance", "all", "watch", "advanced"],
  string: ["filter", "parallel"],
  alias: { 
    u: "unit", 
    i: "integration", 
    p: "performance",
    a: "all", 
    w: "watch",
    f: "filter",
    t: "parallel"
  },
  default: { all: true }
});

async function runUnitTests(filter?: string) {
  console.log(cyan("🧪 Running unit tests in PARALLEL..."));
  console.log(yellow("-----"));
  
  try {
    const baseCmd = ["cargo", "test", "--lib"];
    
    if (filter) {
      baseCmd.push(filter);
    }
    
    // Get the number of CPU cores for maximum parallelism
    const cpuCount = navigator.hardwareConcurrency || 4;
    await $`${baseCmd}`.cwd("backend").env("RUST_TEST_THREADS", cpuCount.toString());
    
    console.log(green("✅ Unit tests completed successfully"));
    return true;
  } catch (error) {
    console.log(red("❌ Unit tests failed"));
    console.error(error);
    return false;
  }
}

async function runPerformanceTests(filter?: string) {
  console.log(magenta("⚡ Running performance tests SEQUENTIALLY (resource-intensive)..."));
  console.log(yellow("-----"));
  
  try {
    const baseCmd = ["cargo", "test", "--test", "tasks_performance"];
    
    if (filter) {
      baseCmd.push(filter);
    }
    
    // Performance tests MUST run sequentially due to resource usage
    await $`${baseCmd}`.cwd("backend").env("RUST_TEST_THREADS", "1");
    
    console.log(green("✅ Performance tests completed successfully"));
    return true;
  } catch (error) {
    console.log(red("❌ Performance tests failed"));
    console.error(error);
    return false;
  }
}

async function runIntegrationTests(filter?: string, parallelThreads = 1, excludePerformance = false) {
  const mode = parallelThreads > 1 ? "LIMITED PARALLEL" : "SEQUENTIAL";
  const excludeMsg = excludePerformance ? " (excluding performance tests)" : "";
  console.log(cyan(`🏗️  Running integration tests ${mode} (${parallelThreads} threads)${excludeMsg}...`));
  console.log(yellow("-----"));
  
  try {
    const baseCmd = ["cargo", "test", "--test"];
    
    if (excludePerformance) {
      // Run all integration tests except performance
      const testFiles = ["health_check", "tasks_api", "tasks_edge_cases", "tasks_operational", "tasks_security", "tasks_concurrency"];
      for (const testFile of testFiles) {
        console.log(blue(`  Running ${testFile}...`));
        const cmd = [...baseCmd, testFile];
        if (filter) {
          cmd.push(filter);
        }
        await $`${cmd}`.cwd("backend").env("RUST_TEST_THREADS", parallelThreads.toString());
      }
    } else {
      baseCmd.push("*");
      if (filter) {
        baseCmd.push(filter);
      }
      await $`${baseCmd}`.cwd("backend").env("RUST_TEST_THREADS", parallelThreads.toString());
    }
    
    console.log(green("✅ Integration tests completed successfully"));
    return true;
  } catch (error) {
    console.log(red("❌ Integration tests failed"));
    console.error(error);
    return false;
  }
}

async function runAllTests(filter?: string, advanced = false) {
  console.log(blue(`🚀 Running ${advanced ? "ADVANCED" : "standard"} test suite...`));
  console.log(yellow("=".repeat(50)));
  
  const startTime = Date.now();
  
  // Run unit tests first (fast parallel execution)
  const unitSuccess = await runUnitTests(filter);
  
  // Only run integration tests if unit tests pass (fail-fast approach)
  if (!unitSuccess) {
    console.log(red("❌ Skipping integration and performance tests due to unit test failures"));
    return false;
  }
  
  // Choose parallelism strategy for integration tests (excluding performance)
  const integrationThreads = advanced ? 2 : 1; // Limited parallelism in advanced mode
  const integrationSuccess = await runIntegrationTests(filter, integrationThreads, true);
  
  if (!integrationSuccess) {
    console.log(red("❌ Skipping performance tests due to integration test failures"));
    return false;
  }
  
  // Run performance tests separately and sequentially (always single-threaded)
  const performanceSuccess = await runPerformanceTests(filter);
  
  const endTime = Date.now();
  const duration = ((endTime - startTime) / 1000).toFixed(2);
  
  if (unitSuccess && integrationSuccess && performanceSuccess) {
    console.log(green(`✅ All tests completed successfully in ${duration}s`));
    if (advanced) {
      console.log(blue(`📊 Advanced mode used ${integrationThreads} threads for integration tests`));
    }
    console.log(magenta("📊 Performance tests run sequentially for resource isolation"));
    return true;
  } else {
    console.log(red(`❌ Test suite failed after ${duration}s`));
    return false;
  }
}

async function watchTests() {
  console.log(blue("👀 Watching for changes... (unit tests only for speed)"));
  console.log(yellow("Press Ctrl+C to stop"));
  
  const cpuCount = navigator.hardwareConcurrency || 4;
  // Use cargo watch for unit tests only during development
  await $`cargo watch -x "test --lib -- --test-threads=${cpuCount}"`.cwd("backend");
}

// Main execution logic
if (args.watch) {
  await watchTests();
} else if (args.unit) {
  const success = await runUnitTests(args.filter);
  Deno.exit(success ? 0 : 1);
} else if (args.performance) {
  const success = await runPerformanceTests(args.filter);
  Deno.exit(success ? 0 : 1);
} else if (args.integration) {
  const parallelThreads = args.parallel ? parseInt(args.parallel) : 1;
  const success = await runIntegrationTests(args.filter, parallelThreads, false);
  Deno.exit(success ? 0 : 1);
} else if (args.all) {
  const success = await runAllTests(args.filter, args.advanced);
  Deno.exit(success ? 0 : 1);
} else {
  console.log(yellow("Usage:"));
  console.log("  deno task test:backend                  # Run all tests optimally");
  console.log("  deno task test:backend --unit           # Run unit tests in parallel");
  console.log("  deno task test:backend --integration    # Run integration tests sequentially");
  console.log("  deno task test:backend --performance    # Run performance tests sequentially");
  console.log("  deno task test:backend --watch          # Watch unit tests during development");
  console.log("  deno task test:backend --filter foo     # Run tests matching 'foo'");
  console.log("  deno task test:backend --parallel 2     # Run integration tests with 2 threads");
  console.log("  deno task test:backend --advanced       # Run advanced test suite");
  console.log("");
  console.log(magenta("Performance tests are always run sequentially due to resource requirements"));
} 