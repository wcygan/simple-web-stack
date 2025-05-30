import { define } from "../../../utils.ts";

// Backend URL using Docker service name for container-to-container communication
const BACKEND_API = "http://backend:3000";

export const handler = define.handlers({
  // GET /api/tasks/:id - Get a specific task
  async GET(ctx) {
    try {
      const id = ctx.params.id;
      const response = await fetch(`${BACKEND_API}/tasks/${id}`);
      const data = await response.json();
      
      return new Response(JSON.stringify(data), {
        status: response.status,
        headers: {
          "Content-Type": "application/json",
        },
      });
    } catch (error) {
      console.error("Failed to fetch task from backend:", error);
      return new Response(
        JSON.stringify({ error: "Failed to fetch task" }),
        {
          status: 500,
          headers: { "Content-Type": "application/json" },
        }
      );
    }
  },

  // PUT /api/tasks/:id - Update a specific task
  async PUT(ctx) {
    try {
      const id = ctx.params.id;
      const body = await ctx.req.text();
      
      const response = await fetch(`${BACKEND_API}/tasks/${id}`, {
        method: "PUT",
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
      console.error("Failed to update task in backend:", error);
      return new Response(
        JSON.stringify({ error: "Failed to update task" }),
        {
          status: 500,
          headers: { "Content-Type": "application/json" },
        }
      );
    }
  },

  // DELETE /api/tasks/:id - Delete a specific task
  async DELETE(ctx) {
    try {
      const id = ctx.params.id;
      const response = await fetch(`${BACKEND_API}/tasks/${id}`, {
        method: "DELETE",
      });
      
      // For DELETE, backend returns 204 No Content with no body
      return new Response(null, {
        status: response.status,
      });
    } catch (error) {
      console.error("Failed to delete task in backend:", error);
      return new Response(
        JSON.stringify({ error: "Failed to delete task" }),
        {
          status: 500,
          headers: { "Content-Type": "application/json" },
        }
      );
    }
  },
}); 