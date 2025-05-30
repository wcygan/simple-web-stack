import { define } from "../../../utils.ts";

// Backend URL using Docker service name for container-to-container communication
const BACKEND_API = "http://backend:3000";

export const handler = define.handlers({
  // GET /api/tasks - List all tasks
  async GET(_ctx) {
    try {
      const response = await fetch(`${BACKEND_API}/tasks`);
      const data = await response.json();

      return new Response(JSON.stringify(data), {
        status: response.status,
        headers: {
          "Content-Type": "application/json",
        },
      });
    } catch (error) {
      console.error("Failed to fetch tasks from backend:", error);
      return new Response(
        JSON.stringify({ error: "Failed to fetch tasks" }),
        {
          status: 500,
          headers: { "Content-Type": "application/json" },
        },
      );
    }
  },

  // POST /api/tasks - Create a new task
  async POST(ctx) {
    try {
      const body = await ctx.req.text();

      const response = await fetch(`${BACKEND_API}/tasks`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body,
      });

      const data = await response.json();

      return new Response(JSON.stringify(data), {
        status: response.status,
        headers: {
          "Content-Type": "application/json",
        },
      });
    } catch (error) {
      console.error("Failed to create task in backend:", error);
      return new Response(
        JSON.stringify({ error: "Failed to create task" }),
        {
          status: 500,
          headers: { "Content-Type": "application/json" },
        },
      );
    }
  },
});
