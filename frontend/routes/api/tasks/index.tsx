import { define } from "../../../utils.ts";

// Backend URL using Docker service name for container-to-container communication
const BACKEND_API = "http://backend:3000";

export const handler = define.handlers({
  // GET /api/tasks - List all tasks with pagination support
  async GET(ctx) {
    try {
      const url = new URL(ctx.req.url);
      const queryParams = url.searchParams;
      
      // Forward pagination and other query parameters to backend
      const backendUrl = `${BACKEND_API}/tasks?${queryParams.toString()}`;
      const response = await fetch(backendUrl);
      
      // Check if response is ok and has JSON content
      if (!response.ok) {
        const errorText = await response.text();
        console.error(`Backend returned error ${response.status}: ${errorText}`);
        return new Response(
          JSON.stringify({ 
            error: `Backend error: ${response.statusText}`,
            details: errorText 
          }),
          {
            status: response.status,
            headers: { "Content-Type": "application/json" },
          }
        );
      }
      
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

      // Check if response is ok
      if (!response.ok) {
        const errorText = await response.text();
        console.error(`Backend returned error ${response.status}: ${errorText}`);
        return new Response(
          JSON.stringify({ 
            error: `Backend error: ${response.statusText}`,
            details: errorText 
          }),
          {
            status: response.status,
            headers: { "Content-Type": "application/json" },
          }
        );
      }

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
