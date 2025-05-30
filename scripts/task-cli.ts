#!/usr/bin/env -S deno run --allow-all
/**
 * Task Management CLI
 *
 * This script provides a command-line interface to interact with the backend task API.
 * It allows users to perform CRUD (Create, Read, Update, Delete) operations on tasks.
 *
 * Prerequisites:
 *  - Deno installed (https://deno.land/)
 *  - Backend API server running and accessible at http://localhost:3000/tasks
 *
 * Usage:
 *  deno task cli <command> [arguments]
 *
 * Commands:
 *  list                             List all tasks.
 *  get <task_id>                    Get a specific task by its ID.
 *  create --title "Task Title"      Create a new task with the given title.
 *  update <task_id> [options]       Update an existing task.
 *    Options:
 *      --title "New Title"        Update the task's title.
 *      --completed <true|false>   Update the task's completion status.
 *  delete <task_id>                 Delete a task by its ID.
 *  help (or any other command)      Display this help message.
 *
 * Examples:
 *  deno task cli list
 *  deno task cli create --title "Buy groceries"
 *  deno task cli get <some_task_id>
 *  deno task cli update <some_task_id> --completed true
 *  deno task cli update <some_task_id> --title "Buy organic groceries" --completed true
 *  deno task cli delete <some_task_id>
 */
import $ from "jsr:@david/dax@^0.39.2";
import { parseArgs } from "jsr:@std/cli@^0.224.0/parse-args";
import { green, yellow, blue, red, cyan } from "jsr:@std/fmt@^0.224.0/colors";

const API_BASE_URL = "http://localhost:3000/tasks";

async function main() {
  const args = parseArgs(Deno.args, {
    string: ["title"],
    boolean: ["completed"],
    alias: {
      t: "title",
      c: "completed",
    },
  });

  const command = args._[0];
  const id = args._[1] as string | undefined;

  console.log(cyan("Task CLI"));
  console.log(yellow("--------------------"));

  try {
    switch (command) {
      case "list":
        await listTasks();
        break;
      case "get":
        if (!id) {
          console.error(red("Error: Task ID is required for 'get' command."));
          Deno.exit(1);
        }
        await getTask(id);
        break;
      case "create":
        if (!args.title) {
          console.error(red("Error: --title is required for 'create' command."));
          Deno.exit(1);
        }
        await createTask(args.title);
        break;
      case "update":
        if (!id) {
          console.error(red("Error: Task ID is required for 'update' command."));
          Deno.exit(1);
        }
        if (!args.title && args.completed === undefined) {
          console.error(red("Error: At least --title or --completed must be provided for 'update' command."));
          Deno.exit(1);
        }
        await updateTask(id, args.title, args.completed);
        break;
      case "delete":
        if (!id) {
          console.error(red("Error: Task ID is required for 'delete' command."));
          Deno.exit(1);
        }
        await deleteTask(id);
        break;
      default:
        printHelp();
    }
  } catch (error) {
    const err = error as Error;
    console.error(red(`Error: ${err.message}`));
    if (err.cause) {
      console.error(red(`Cause: ${(err.cause as Error).message || err.cause}`));
    }
    Deno.exit(1);
  }
}

function printHelp() {
  console.log(blue("Available commands:"));
  console.log(green("  list") + "                     - List all tasks");
  console.log(green("  get <id>") + "                - Get a specific task by ID");
  console.log(green("  create --title <title>") + "   - Create a new task");
  console.log(green("  update <id> [--title <new_title>] [--completed <true|false>]") + " - Update a task");
  console.log(green("  delete <id>") + "              - Delete a task by ID");
  console.log("\nExample:");
  console.log(yellow("  deno task cli create --title \"My New Task\""));
  console.log(yellow("  deno task cli list"));
  console.log(yellow("  deno task cli update <task_id> --completed true"));
}

// Placeholder functions for API interactions
async function listTasks() {
  try {
    const response = await fetch(API_BASE_URL);
    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`API Error (${response.status}): ${errorText || response.statusText}`);
    }
    const tasks = await response.json();

    if (!tasks || tasks.length === 0) {
      console.log(yellow("No tasks found."));
      return;
    }

    console.log(blue("Current Tasks:"));
    tasks.forEach((task: any) => {
      console.log(
        `  [${task.completed ? green("✔") : red("✘")}] ${task.title} (ID: ${yellow(task.id)})`
      );
    });
  } catch (e) {
    const error = e as Error;
    console.error(red(`Failed to list tasks: ${error.message}`));
    throw error; // Re-throw to be caught by main error handler
  }
}

async function getTask(id: string) {
  try {
    const response = await fetch(`${API_BASE_URL}/${id}`);
    if (!response.ok) {
      if (response.status === 404) {
        console.error(red(`Error: Task with ID "${id}" not found.`));
        Deno.exit(1);
      }
      const errorText = await response.text();
      throw new Error(`API Error (${response.status}): ${errorText || response.statusText}`);
    }
    const task = await response.json();
    console.log(blue("Task details:"));
    console.log(
      `  [${task.completed ? green("✔") : red("✘")}] ${task.title} (ID: ${yellow(task.id)})`
    );
    // console.log(blue(`  Created At: ${task.created_at}`)); // Assuming created_at and updated_at are available
    // console.log(blue(`  Updated At: ${task.updated_at}`));
  } catch (e) {
    const error = e as Error;
    console.error(red(`Failed to get task: ${error.message}`));
    throw error;
  }
}

async function createTask(title: string) {
  try {
    const response = await fetch(API_BASE_URL, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ title }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`API Error (${response.status}): ${errorText || response.statusText}`);
    }

    const newTask = await response.json();
    console.log(green("Task created successfully:"));
    console.log(
      `  [${newTask.completed ? green("✔") : red("✘")}] ${newTask.title} (ID: ${yellow(newTask.id)})`
    );
  } catch (e) {
    const error = e as Error;
    console.error(red(`Failed to create task: ${error.message}`));
    throw error;
  }
}

async function updateTask(id: string, title?: string, completed?: boolean) {
  try {
    const payload: { title?: string; completed?: boolean } = {};
    if (title) {
      payload.title = title;
    }
    if (completed !== undefined) {
      payload.completed = completed;
    }

    if (Object.keys(payload).length === 0) {
      console.log(yellow("No changes specified for update."));
      return;
    }

    const response = await fetch(`${API_BASE_URL}/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      if (response.status === 404) {
        console.error(red(`Error: Task with ID "${id}" not found.`));
        Deno.exit(1);
      }
      const errorText = await response.text();
      throw new Error(`API Error (${response.status}): ${errorText || response.statusText}`);
    }

    const updatedTask = await response.json();
    console.log(green("Task updated successfully:"));
    console.log(
      `  [${updatedTask.completed ? green("✔") : red("✘")}] ${updatedTask.title} (ID: ${yellow(updatedTask.id)})`
    );
  } catch (e) {
    const error = e as Error;
    console.error(red(`Failed to update task: ${error.message}`));
    throw error;
  }
}

async function deleteTask(id: string) {
  try {
    const response = await fetch(`${API_BASE_URL}/${id}`, {
      method: "DELETE",
    });

    if (!response.ok) {
      if (response.status === 404) {
        console.error(red(`Error: Task with ID "${id}" not found.`));
        Deno.exit(1);
      }
      const errorText = await response.text();
      throw new Error(`API Error (${response.status}): ${errorText || response.statusText}`);
    }

    // DELETE typically returns 204 No Content, so no JSON body to parse.
    console.log(green(`Task with ID "${id}" deleted successfully.`));
  } catch (e) {
    const error = e as Error;
    console.error(red(`Failed to delete task: ${error.message}`));
    throw error;
  }
}

if (import.meta.main) {
  await main();
} 