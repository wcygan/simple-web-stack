# Deno Fresh 2 Essentials

This document provides a concise overview of Deno Fresh 2, highlighting its core concepts, features, and essential structure for developers.

## What is Fresh?

Fresh is a next-generation full-stack web framework for Deno, built for speed, reliability, and simplicity. Key characteristics include:

-   **Just-In-Time (JIT) Rendering**: Pages are rendered on demand, typically on edge servers.
-   **Islands Architecture**: Client-side interactivity is opt-in via "islands" of Preact components. Most of the page ships no JavaScript by default.
-   **Zero Runtime Overhead**: The framework itself aims to add minimal JavaScript to the client.
-   **TypeScript First**: Built-in support for TypeScript.
-   **File-System Routing**: Routes are automatically created based on the `routes/` directory structure.

## Key Features

-   Minimal JavaScript shipped to the client by default.
-   Interactive components ("islands") are hydrated independently.
-   No build step is necessary during development (JIT compilation).
-   Server-side rendering (SSR) by default for fast initial page loads.
-   Optional Ahead-of-Time (AOT) builds for optimized production assets.

## Getting Started

1.  **Scaffold a new project**:
    ```sh
    deno run -A -r https://fresh.deno.dev my-fresh-project
    ```
2.  **Navigate to the project directory**:
    ```sh
    cd my-fresh-project
    ```
3.  **Start the development server**:
    ```sh
    deno task start
    ```
    This typically runs `dev.ts`, which manages the development server and asset building.

## Core Project Structure (Fresh 2)

A typical Fresh 2 project includes the following key directories and files:

-   **`routes/`**: Contains page components and API handlers.
    -   File names directly map to URL paths (e.g., `routes/about.tsx` -> `/about`).
    -   `_app.tsx`: Defines the global application wrapper (HTML structure, global styles).
    -   `_error.tsx`: A unified error page for handling 404s and other HTTP errors.
    -   `_middleware.ts`: For defining middleware that applies to routes within its directory and subdirectories.
-   **`islands/`**: Holds interactive Preact components ("islands"). These are the only components that will send JavaScript to the client.
-   **`components/`**: For shared, non-interactive Preact components rendered on the server.
-   **`static/`**: For static assets like images, CSS files, and fonts.
-   **`dev.ts`**: The entry point for development. It uses `fresh/dev.Builder` to configure the dev server, plugins (like Tailwind CSS), and the build process.
-   **`main.ts`**: The entry point for production deployments. It initializes the `App`, registers middleware, and sets up file-system routing.
-   **`deno.json`**: Deno's configuration file, managing tasks, import maps (dependencies), and compiler options (e.g., JSX settings).

**Notable Change in Fresh 2**: `fresh.config.ts` and the auto-generated `fresh.gen.ts` (manifest file) are no longer used by default. Island and route discovery is handled dynamically by `fresh/dev.Builder` in `dev.ts` and `fsRoutes` in `main.ts`.

## Core Concepts

-   **Server Components by Default**: All components in `routes/` and `components/` are rendered on the server and do not ship JavaScript to the client unless they include an island.
-   **Progressive Enhancement**: Start with server-rendered HTML and add interactivity where needed with islands.
-   **Route Handlers**: Server-side logic, including data fetching, form submissions, and API endpoint creation, is implemented within handler functions in route files.
-   **Ahead-of-Time (AOT) Build for Production**: While development uses JIT, production deployments should use a build step (e.g., `deno task build` or `deno run -A dev.ts build`) to optimize assets.
-   **Deployment**:
    -   Optimized for [Deno Deploy](https://deno.com/deploy).
    -   Can be self-hosted (e.g., using Docker). When self-hosting, it's crucial to set the `DENO_DEPLOYMENT_ID` environment variable (e.g., to a Git commit hash) to ensure correct asset caching.

## Key Changes from Fresh 1.x to Fresh 2.x (Summary)

-   **Configuration Model**: Shifted from `fresh.config.ts` and `fresh.gen.ts` to direct configuration in `dev.ts` (using `Builder`) and `main.ts` (using `new App()` and `fsRoutes`).
-   **Error Handling**: `routes/_error.tsx` is now the unified error page, replacing separate `_404.tsx` and `_500.tsx`.
-   **`<Head>` Component**: The `<Head>` component for injecting tags from anywhere is removed. Head elements are typically managed within `routes/_app.tsx`, potentially using data passed via `ctx.state`.
-   **Build Step**: An AOT build step is mandatory for production deployments to generate optimized assets.
-   **API Signature Unification**: Middleware, handlers, and async route components now generally receive a single `ctx` (or `props`) argument, with the `Request` object available as `ctx.req`.

## Further Learning

For detailed information and advanced topics, refer to the official Fresh documentation:
-   [Fresh Website & Docs](https://fresh.deno.dev/)
