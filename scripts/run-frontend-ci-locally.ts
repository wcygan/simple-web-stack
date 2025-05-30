#!/usr/bin/env -S deno run --allow-all
import $ from "jsr:@david/dax";
import { parseArgs } from "@std/cli/parse-args";
import { cyan, yellow, blue, green } from "@std/fmt/colors";

const WORKFLOW_FILE = ".github/workflows/frontend-ci.yml";
const JOB_NAME = "build_and_test_frontend"; // Must match the job name in the workflow file

// Parse command line arguments
const args = parseArgs(Deno.args, {
  alias: { e: "eventType" }
});

const eventType = args.eventType || args._[0] || "pull_request";

console.log(cyan("Running Frontend CI locally using act..."));
console.log(`Workflow: ${WORKFLOW_FILE}`);
console.log(`Job: ${JOB_NAME}`);
console.log(`Event Type: ${eventType}`);

try {
  const actrcContent = await $`cat .actrc`.text().catch(() => "Not found or empty");
  console.log(`Using .actrc: ${actrcContent}`);
} catch {
  console.log("Using .actrc: Not found or empty");
}

console.log(yellow("-----------------------------------------------------"));

// For 'act' to correctly simulate path filtering on pull_request events,
// it needs to detect changes in the specified paths relative to the target branch (e.g., main).
// Ensure you are on a feature branch with relevant commits or create dummy changes.
if (eventType === "pull_request") {
  console.log(blue("INFO: Simulating a 'pull_request' event."));
  console.log("For path filters ('paths:') to work correctly with 'act', ensure you have committed");
  console.log("changes in the 'frontend/' directory on your current branch that are not on 'main'.");
  console.log(green("Example to create dummy changes for testing:"));
  console.log("  git checkout main && git pull origin main");
  console.log("  git checkout -b temp-frontend-test");
  console.log("  mkdir -p frontend/act_test_dummy && touch frontend/act_test_dummy/file.txt");
  console.log("  git add frontend/act_test_dummy/file.txt");
  console.log("  git commit -m 'feat: dummy change in frontend for act testing'");
  console.log("Then run this script: deno task ci:frontend pull_request");
  console.log("After testing, you can clean up: git checkout main && git branch -D temp-frontend-test");
  console.log(yellow("-----------------------------------------------------"));
  
  await $`act pull_request --job ${JOB_NAME} -W ${WORKFLOW_FILE}`;
} else {
  console.log(blue("INFO: Simulating a 'push' to main event."));
  console.log("Path filters are not applied for 'push' to main; the workflow should always run.");
  console.log(yellow("-----------------------------------------------------"));
  
  await $`act push --job ${JOB_NAME} -W ${WORKFLOW_FILE}`;
}

console.log(yellow("-----------------------------------------------------"));
console.log(green("Frontend CI local run finished.")); 