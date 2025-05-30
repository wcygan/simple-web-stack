import TodoApp from "../islands/TodoApp.tsx";

export default function Home() {
  return (
    <div class="container mx-auto px-4 py-8 max-w-2xl">
      <header class="text-center mb-12">
        <h1 class="text-5xl font-bold text-gray-700">
          My Todo List
        </h1>
      </header>
      <main>
        <TodoApp />
      </main>
      <footer class="text-center mt-12 text-gray-600">
        <p>&copy; {new Date().getFullYear()} Todo App. Built with Fresh.</p>
      </footer>
    </div>
  );
}
