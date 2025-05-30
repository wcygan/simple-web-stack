#!/usr/bin/env -S deno run --allow-read --allow-run --allow-env --allow-net

import { parseArgs } from "@std/cli/parse-args";
import { red, green, yellow, cyan } from "@std/fmt/colors";
import { ensureDir } from "@std/fs/ensure-dir";

const GITHUB_DIR = ".github";
const WORKFLOWS_DIR = `${GITHUB_DIR}/workflows`;
const FRONTEND_WORKFLOW_FILE = "frontend-ci.yml";
const BACKEND_WORKFLOW_FILE = "backend-ci.yml";
const ACTRC_FILE = ".actrc";

interface ScriptOptions {
  help: boolean;
  verbose: boolean;
  "dry-run": boolean;
  frontend: boolean;
  backend: boolean;
  event?: string;
}

async function commandExists(command: string): Promise<boolean> {
  try {
    const process = new Deno.Command(command, {
      args: ["--version"],
      stdout: "null",
      stderr: "null",
    });
    const status = await process.output();
    return status.success;
  } catch (e: unknown) {
    if (e instanceof Deno.errors.NotFound) {
      return false;
    }
    // Other errors might indicate the command exists but --version failed,
    // which is still a good sign for existence.
    return true;
  }
}

async function checkPrerequisites(): Promise<boolean> {
  const prerequisites = [
    { name: "Deno", check: async () => await commandExists("deno") },
    { name: "Docker", check: async () => await commandExists("docker") },
    { name: "act", check: async () => await commandExists("act") },
  ];

  let allOk = true;
  console.log(cyan("Checking prerequisites:"));
  for (const prereq of prerequisites) {
    if (await prereq.check()) {
      console.log(green(`  ✔ ${prereq.name} is installed.`));
    } else {
      console.log(red(`  ✖ ${prereq.name} is not installed or not in PATH.`));
      allOk = false;
    }
  }

  if (!allOk) {
    console.log(yellow("\nPlease install missing prerequisites and try again."));
  }
  return allOk;
}

async function runAct(
  workflowFile: string,
  eventName: string,
  isDryRun: boolean,
  isVerbose: boolean,
): Promise<void> {
  const workflowPath = `${WORKFLOWS_DIR}/${workflowFile}`;
  console.log(cyan(`\nRunning ${workflowPath} for event '${eventName}'...`));

  const actArgs = [eventName, "-W", workflowPath];
  if (isVerbose) {
    actArgs.push("-v");
  }
  
  // Check for .actrc in the current working directory (workspace root)
  if (await Deno.stat(ACTRC_FILE).then(s => s.isFile).catch(() => false)) {
    console.log(yellow(`  Using ${ACTRC_FILE} from workspace root for act configuration.`));
  } else {
    console.log(yellow(`  ${ACTRC_FILE} not found in workspace root. Consider creating one for optimized runs.`));
    if (Deno.build.os === "darwin") {
      actArgs.push("--container-architecture", "linux/amd64");
      console.log(yellow("  Using --container-architecture linux/amd64 for macOS as a default. Create .actrc for custom settings."));
    }
  }

  if (isDryRun) {
    console.log(yellow(`  Dry run: Would execute 'act ${actArgs.join(" ")}'`));
    return;
  }

  const command = new Deno.Command("act", {
    args: actArgs,
    stdout: "inherit",
    stderr: "inherit",
    // cwd is implicitly the workspace root, as the Deno script is run from there.
    // Explicitly: cwd: "."
  });

  try {
    const status = await command.output();
    if (status.success) {
      console.log(green(`✔ Successfully ran ${workflowPath}`));
    } else {
      console.log(red(`✖ Failed to run ${workflowPath}. Exit code: ${status.code}`));
    }
  } catch (error) {
    console.error(red(`  Error executing act for ${workflowPath}:`), error);
  }
}

function showHelp(): void {
  console.log(`
Usage: deno run --allow-read --allow-run --allow-env simple-web-stack/scripts/run-ci.ts [options]

Description:
  Runs GitHub Actions workflows (frontend-ci.yml and/or backend-ci.yml) locally using 'act'.
  This script must be executed from the root of the 'development-workspace'.

Options:
  --help              Show this help message.
  --verbose           Enable verbose output for act.
  --dry-run           Show commands that would be executed without running them.
  --frontend          Only run the frontend workflow.
  --backend           Only run the backend workflow.
  --event <name>      Specify the event to trigger (e.g., 'push', 'pull_request').
                      Default: 'pull_request'.

Prerequisites:
  - Deno
  - Docker (must be running)
  - act (https://github.com/nektos/act)

Examples:
  # Run both workflows (default event: pull_request) from the workspace root
  deno task ci

  # Run only the frontend workflow for a push event
  deno task ci -- --frontend --event push 
  # Note: Use '--' to pass flags to the script when using 'deno task'
  `);
}

async function main(): Promise<void> {
  const args = parseArgs(Deno.args, {
    boolean: ["help", "verbose", "dry-run", "frontend", "backend"],
    string: ["event"],
    alias: { h: "help", v: "verbose" },
  }) as ScriptOptions;

  if (args.help) {
    showHelp();
    return;
  }

  if (!(await checkPrerequisites())) {
    Deno.exit(1);
  }

  const eventName = args.event || "pull_request";
  let runFrontend = args.frontend;
  let runBackend = args.backend;

  if (!runFrontend && !runBackend) {
    runFrontend = true;
    runBackend = true;
  }
  
  // Ensure .github/workflows directory exists (relative to workspace root)
  try {
    await ensureDir(WORKFLOWS_DIR); // WORKFLOWS_DIR is now ./.github/workflows
  } catch (e: unknown) {
    console.error(red(`Error ensuring ${WORKFLOWS_DIR} directory exists: ${(e instanceof Error ? e.message : String(e))}`),
    );
    Deno.exit(1);
  }

  if (runFrontend) {
    await runAct(FRONTEND_WORKFLOW_FILE, eventName, args["dry-run"], args.verbose);
  }

  if (runBackend) {
    await runAct(BACKEND_WORKFLOW_FILE, eventName, args["dry-run"], args.verbose);
  }

  console.log(cyan("\nLocal CI run finished."));
}

if (import.meta.main) {
  main().catch((err: unknown) => {
    console.error(red("Unhandled error in main:"), (err instanceof Error ? err.message : String(err)));
    Deno.exit(1);
  });
} 