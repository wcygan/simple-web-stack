import { useState } from "preact/hooks";
import { signal } from "@preact/signals";
import { AuthService } from "../auth.ts";
import type { LoginRequest } from "../types.ts";

// Auth state signals
export const authState = signal({
  user: AuthService.getUser(),
  token: AuthService.getToken(),
  isAuthenticated: AuthService.isAuthenticated(),
  isLoading: false,
  error: null as string | null,
});

export default function LoginForm() {
  const [formData, setFormData] = useState<LoginRequest>({
    email: "",
    password: "",
  });

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    
    if (!formData.email || !formData.password) {
      authState.value = { ...authState.value, error: "Please fill in all fields" };
      return;
    }

    authState.value = { ...authState.value, isLoading: true, error: null };

    try {
      const response = await AuthService.login(formData);
      authState.value = {
        user: response.user,
        token: response.token,
        isAuthenticated: true,
        isLoading: false,
        error: null,
      };
      
      // Redirect to main app or refresh page
      window.location.href = "/tasks";
    } catch (error) {
      authState.value = {
        ...authState.value,
        isLoading: false,
        error: error instanceof Error ? error.message : "Login failed",
      };
    }
  };

  const handleInputChange = (field: keyof LoginRequest) => (e: Event) => {
    const target = e.target as HTMLInputElement;
    setFormData(prev => ({
      ...prev,
      [field]: target.value,
    }));
  };

  return (
    <div class="max-w-md mx-auto mt-8 p-6 bg-white rounded-lg shadow-md">
      <h2 class="text-2xl font-bold mb-6 text-center text-gray-800">Login</h2>
      
      <form onSubmit={handleSubmit} class="space-y-4">
        <div>
          <label htmlFor="email" class="block text-sm font-medium text-gray-700 mb-1">
            Email
          </label>
          <input
            type="email"
            id="email"
            value={formData.email}
            onInput={handleInputChange("email")}
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="Enter your email"
            disabled={authState.value.isLoading}
          />
        </div>

        <div>
          <label htmlFor="password" class="block text-sm font-medium text-gray-700 mb-1">
            Password
          </label>
          <input
            type="password"
            id="password"
            value={formData.password}
            onInput={handleInputChange("password")}
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="Enter your password"
            disabled={authState.value.isLoading}
          />
        </div>

        {authState.value.error && (
          <div class="p-3 bg-red-100 border border-red-400 text-red-700 rounded">
            {authState.value.error}
          </div>
        )}

        <button
          type="submit"
          disabled={authState.value.isLoading}
          class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed"
        >
          {authState.value.isLoading ? "Logging in..." : "Login"}
        </button>
      </form>

      <p class="mt-4 text-center text-sm text-gray-600">
        Don't have an account?{" "}
        <a href="/register" class="text-blue-600 hover:text-blue-500">
          Sign up
        </a>
      </p>
    </div>
  );
}