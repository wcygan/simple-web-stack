import TodoApp from "../islands/TodoApp.tsx";

export default function Home() {
  return (
    <div class="px-4 py-8 mx-auto max-w-screen-md">
      <header class="mb-8 text-center">
        <h1 class="text-4xl font-bold text-gray-800">My Todo List</h1>
      </header>
      <main>
        <TodoApp />
      </main>
      <footer class="mt-12 text-center text-gray-500 text-sm">
        <p>Powered by Deno Fresh & Tailwind CSS</p>
      </footer>
    </div>
  );
}
