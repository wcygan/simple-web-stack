#!/usr/bin/env -S deno run --allow-read --allow-write

import { brightBlue, brightGreen, brightRed, brightYellow, gray } from "@std/fmt/colors";

interface InstanceStatus {
  status: "not-started" | "active" | "blocked" | "completed";
  currentTask: string;
  completedTasks: string[];
  blockers: string[];
  progress: number;
  stream: string;
  priority: string;
}

interface SyncPoint {
  time: string;
  status: "pending" | "ready" | "completed";
  streams: string[];
}

interface StatusFile {
  lastUpdate: string;
  projectName: string;
  totalStreams: number;
  instances: Record<string, InstanceStatus>;
  synchronizationPoints: Record<string, SyncPoint>;
}

// Read the status file
async function readStatus(): Promise<StatusFile> {
  try {
    const content = await Deno.readTextFile("/tmp/claude-scratch/status.json");
    return JSON.parse(content);
  } catch (error) {
    console.error(brightRed("Error reading status file:"), error.message);
    Deno.exit(1);
  }
}

// Read the coordination plan
async function readCoordinationPlan(): Promise<string> {
  try {
    return await Deno.readTextFile("/tmp/claude-coordination.md");
  } catch (error) {
    console.error(brightRed("Error reading coordination plan:"), error.message);
    return "";
  }
}

// Calculate overall progress
function calculateOverallProgress(instances: Record<string, InstanceStatus>): number {
  const progresses = Object.values(instances).map(i => i.progress);
  return progresses.reduce((a, b) => a + b, 0) / progresses.length;
}

// Format status with color
function formatStatus(status: string): string {
  switch (status) {
    case "active": return brightGreen(status);
    case "blocked": return brightRed(status);
    case "completed": return brightBlue(status);
    default: return gray(status);
  }
}

// Format priority with color
function formatPriority(priority: string): string {
  switch (priority) {
    case "critical": return brightRed(priority.toUpperCase());
    case "high": return brightYellow(priority.toUpperCase());
    case "medium": return brightBlue(priority);
    default: return gray(priority);
  }
}

// Progress bar
function progressBar(progress: number, width: number = 20): string {
  const filled = Math.round((progress / 100) * width);
  const empty = width - filled;
  return `[${"█".repeat(filled)}${" ".repeat(empty)}] ${progress}%`;
}

// Main display function
async function displayProgress() {
  const status = await readStatus();
  
  console.clear();
  console.log(brightBlue("=".repeat(80)));
  console.log(brightBlue("Claude Multi-Instance Coordination Progress"));
  console.log(brightBlue("=".repeat(80)));
  console.log();
  
  // Project info
  console.log(`Project: ${brightGreen(status.projectName)}`);
  console.log(`Last Update: ${gray(new Date(status.lastUpdate).toLocaleString())}`);
  console.log(`Total Streams: ${status.totalStreams}`);
  console.log(`Overall Progress: ${progressBar(calculateOverallProgress(status.instances))}`);
  console.log();
  
  // Instance statuses
  console.log(brightBlue("Stream Status:"));
  console.log("-".repeat(80));
  
  for (const [name, instance] of Object.entries(status.instances)) {
    console.log(`\n${brightGreen(name)} (Stream ${instance.stream})`);
    console.log(`  Status: ${formatStatus(instance.status)}`);
    console.log(`  Priority: ${formatPriority(instance.priority)}`);
    console.log(`  Progress: ${progressBar(instance.progress)}`);
    console.log(`  Current: ${instance.currentTask}`);
    
    if (instance.completedTasks.length > 0) {
      console.log(`  Completed: ${instance.completedTasks.length} tasks`);
      for (const task of instance.completedTasks) {
        console.log(`    ✓ ${gray(task)}`);
      }
    }
    
    if (instance.blockers.length > 0) {
      console.log(brightRed(`  Blockers:`));
      for (const blocker of instance.blockers) {
        console.log(`    ⚠ ${blocker}`);
      }
    }
  }
  
  // Synchronization points
  console.log("\n" + brightBlue("Synchronization Points:"));
  console.log("-".repeat(80));
  
  for (const [name, sync] of Object.entries(status.synchronizationPoints)) {
    const statusColor = sync.status === "completed" ? brightGreen : 
                       sync.status === "ready" ? brightYellow : gray;
    console.log(`\n${name} (${sync.time})`);
    console.log(`  Status: ${statusColor(sync.status)}`);
    console.log(`  Streams: ${sync.streams.join(", ")}`);
  }
  
  // Active blockers summary
  const blockedInstances = Object.entries(status.instances)
    .filter(([_, i]) => i.status === "blocked");
  
  if (blockedInstances.length > 0) {
    console.log("\n" + brightRed("⚠ ACTIVE BLOCKERS:"));
    console.log("-".repeat(80));
    for (const [name, instance] of blockedInstances) {
      console.log(`${name}: ${instance.blockers.join(", ")}`);
    }
  }
  
  // Quick stats
  const stats = {
    active: Object.values(status.instances).filter(i => i.status === "active").length,
    blocked: Object.values(status.instances).filter(i => i.status === "blocked").length,
    completed: Object.values(status.instances).filter(i => i.status === "completed").length,
    notStarted: Object.values(status.instances).filter(i => i.status === "not-started").length,
  };
  
  console.log("\n" + brightBlue("Quick Stats:"));
  console.log("-".repeat(80));
  console.log(`Active: ${brightGreen(stats.active.toString())} | ` +
              `Blocked: ${brightRed(stats.blocked.toString())} | ` +
              `Completed: ${brightBlue(stats.completed.toString())} | ` +
              `Not Started: ${gray(stats.notStarted.toString())}`);
  
  console.log("\n" + gray("Refresh: deno task check:coordination"));
}

// Watch mode
async function watchProgress() {
  await displayProgress();
  
  // Set up file watcher
  const watcher = Deno.watchFs(["/tmp/claude-scratch/status.json"]);
  
  console.log("\n" + gray("Watching for updates... (Ctrl+C to exit)"));
  
  for await (const event of watcher) {
    if (event.kind === "modify") {
      await new Promise(resolve => setTimeout(resolve, 100)); // Debounce
      await displayProgress();
      console.log("\n" + gray("Watching for updates... (Ctrl+C to exit)"));
    }
  }
}

// CLI argument handling
const args = Deno.args;
if (args.includes("--watch") || args.includes("-w")) {
  await watchProgress();
} else {
  await displayProgress();
  
  // Also create other helper files
  await Deno.writeTextFile("/tmp/claude-scratch/tasks.md", `# Task Updates

## Stream A: Pagination Backend
- [ ] Add PaginationParams and PaginatedResponse to models.rs
- [ ] Implement pagination logic in routes.rs
- [ ] Add count_tasks function to db.rs
- [ ] Create database indexes
- [ ] Write unit tests
- [ ] Performance testing

## Stream B: Pagination Frontend & Hot-Reload
- [ ] Add pagination types to types.ts
- [ ] Create pagination utilities
- [ ] Update API proxy
- [ ] Hot-reloading Docker setup
- [ ] Create docker-compose.override.yml
- [ ] Update deno.json tasks

## Stream C: Authentication
- [ ] Design user database schema
- [ ] Create migrations
- [ ] JWT implementation
- [ ] Auth middleware
- [ ] Login/register endpoints
- [ ] User-scoped tasks
- [ ] Frontend auth forms

## Stream D: Search & Filtering
- [ ] Backend search parameters
- [ ] Title search implementation
- [ ] Status filtering
- [ ] Search UI components
- [ ] Debounced search
- [ ] Search state management
- [ ] Integration tests

## Stream E: Testing & Documentation
- [ ] Playwright setup
- [ ] OpenAPI specification
- [ ] TypeScript SDK generation
- [ ] E2E tests
- [ ] Visual regression tests
- [ ] API documentation
- [ ] Performance tests

## Stream F: UI/UX Enhancements
- [ ] Dark mode theme
- [ ] Keyboard shortcuts
- [ ] Drag-and-drop reordering
- [ ] Task priorities
- [ ] Bulk operations
- [ ] Loading states
- [ ] Mobile gestures
`);
  
  await Deno.writeTextFile("/tmp/claude-scratch/integration.md", `# Integration Notes

## Type Definitions Coordination
- Pagination types defined in Stream B must match Stream A backend models
- Auth types from Stream C need to be integrated with existing Task type
- Search parameters from Stream D need type definitions

## API Endpoint Coordination
- /api/tasks - Enhanced by Streams A (pagination) and D (search)
- /api/auth/* - New endpoints from Stream C
- /api/users/* - New endpoints from Stream C

## Frontend State Coordination
- TodoApp.tsx modifications needed from Streams B, D, F
- New auth state management from Stream C
- Search state from Stream D

## Merge Order Recommendations
1. Stream A & B together (pagination)
2. Stream C (auth) - standalone
3. Stream D (search) - depends on pagination
4. Stream E (testing) - after core features
5. Stream F (UI) - can merge anytime

## Known Integration Points
- [ ] Pagination and Search both modify task list API
- [ ] Auth adds user_id to all task operations
- [ ] UI enhancements need to work with all new features
`);
  
  console.log("\n" + gray("Use --watch or -w flag to monitor changes"));
}