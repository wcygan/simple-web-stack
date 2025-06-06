import { useState } from "preact/hooks";
import { signal } from "@preact/signals";
import { AuthService } from "../auth.ts";
import type { RegisterRequest } from "../types.ts";

// Reuse the same auth state signal from LoginForm
export const authState = signal({
  user: AuthService.getUser(),
  token: AuthService.getToken(),
  isAuthenticated: AuthService.isAuthenticated(),
  isLoading: false,
  error: null as string | null,
});

export default function RegisterForm() {
  const [formData, setFormData] = useState<RegisterRequest>({
    email: "",
    password: "",
  });
  const [confirmPassword, setConfirmPassword] = useState("");

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    
    if (!formData.email || !formData.password || !confirmPassword) {
      authState.value = { ...authState.value, error: "Please fill in all fields" };
      return;
    }

    if (formData.password !== confirmPassword) {
      authState.value = { ...authState.value, error: "Passwords do not match" };
      return;
    }

    if (formData.password.length < 8) {
      authState.value = { ...authState.value, error: "Password must be at least 8 characters long" };
      return;
    }

    authState.value = { ...authState.value, isLoading: true, error: null };

    try {
      const response = await AuthService.register(formData);
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
        error: error instanceof Error ? error.message : "Registration failed",
      };
    }
  };

  const handleInputChange = (field: keyof RegisterRequest) => (e: Event) => {
    const target = e.target as HTMLInputElement;
    setFormData(prev => ({
      ...prev,
      [field]: target.value,
    }));
  };

  const handleConfirmPasswordChange = (e: Event) => {
    const target = e.target as HTMLInputElement;
    setConfirmPassword(target.value);
  };

  return (
    <div class="max-w-md mx-auto mt-8 p-6 bg-white rounded-lg shadow-md">
      <h2 class="text-2xl font-bold mb-6 text-center text-gray-800">Sign Up</h2>
      
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
            placeholder="Enter your password (min 8 characters)"
            disabled={authState.value.isLoading}
          />
        </div>

        <div>
          <label htmlFor="confirmPassword" class="block text-sm font-medium text-gray-700 mb-1">
            Confirm Password
          </label>
          <input
            type="password"
            id="confirmPassword"
            value={confirmPassword}
            onInput={handleConfirmPasswordChange}
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="Confirm your password"
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
          {authState.value.isLoading ? "Creating account..." : "Sign Up"}
        </button>
      </form>

      <p class="mt-4 text-center text-sm text-gray-600">
        Already have an account?{" "}
        <a href="/login" class="text-blue-600 hover:text-blue-500">
          Log in
        </a>
      </p>
    </div>
  );
}