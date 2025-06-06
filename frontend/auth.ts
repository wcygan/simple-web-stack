import { AuthResponse, LoginRequest, RegisterRequest, User } from "./types.ts";

const API_BASE_URL = "http://localhost:3000";

export class AuthService {
  private static TOKEN_KEY = "auth_token";
  private static USER_KEY = "auth_user";

  // Local storage helpers
  static getToken(): string | null {
    if (typeof localStorage === "undefined") return null;
    return localStorage.getItem(this.TOKEN_KEY);
  }

  static setToken(token: string): void {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(this.TOKEN_KEY, token);
    }
  }

  static removeToken(): void {
    if (typeof localStorage !== "undefined") {
      localStorage.removeItem(this.TOKEN_KEY);
    }
  }

  static getUser(): User | null {
    if (typeof localStorage === "undefined") return null;
    const userJson = localStorage.getItem(this.USER_KEY);
    return userJson ? JSON.parse(userJson) : null;
  }

  static setUser(user: User): void {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(this.USER_KEY, JSON.stringify(user));
    }
  }

  static removeUser(): void {
    if (typeof localStorage !== "undefined") {
      localStorage.removeItem(this.USER_KEY);
    }
  }

  static isAuthenticated(): boolean {
    return !!this.getToken() && !!this.getUser();
  }

  static clear(): void {
    this.removeToken();
    this.removeUser();
  }

  // API calls
  static async register(data: RegisterRequest): Promise<AuthResponse> {
    const response = await fetch(`${API_BASE_URL}/auth/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error || "Registration failed");
    }

    const authResponse: AuthResponse = await response.json();
    
    // Store auth data
    this.setToken(authResponse.token);
    this.setUser(authResponse.user);
    
    return authResponse;
  }

  static async login(data: LoginRequest): Promise<AuthResponse> {
    const response = await fetch(`${API_BASE_URL}/auth/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error || "Login failed");
    }

    const authResponse: AuthResponse = await response.json();
    
    // Store auth data
    this.setToken(authResponse.token);
    this.setUser(authResponse.user);
    
    return authResponse;
  }

  static async logout(): Promise<void> {
    const token = this.getToken();
    
    if (token) {
      try {
        await fetch(`${API_BASE_URL}/auth/logout`, {
          method: "POST",
          headers: {
            "Authorization": `Bearer ${token}`,
          },
        });
      } catch (error) {
        console.warn("Logout request failed:", error);
        // Continue with local logout even if server request fails
      }
    }

    // Clear local storage
    this.clear();
  }

  static async getProfile(): Promise<User> {
    const token = this.getToken();
    
    if (!token) {
      throw new Error("No authentication token");
    }

    const response = await fetch(`${API_BASE_URL}/auth/profile`, {
      method: "GET",
      headers: {
        "Authorization": `Bearer ${token}`,
      },
    });

    if (!response.ok) {
      if (response.status === 401) {
        // Token is invalid, clear local storage
        this.clear();
        throw new Error("Authentication expired");
      }
      const error = await response.json();
      throw new Error(error.error || "Failed to get profile");
    }

    const user: User = await response.json();
    
    // Update stored user data
    this.setUser(user);
    
    return user;
  }

  // Helper to make authenticated API requests
  static async fetchWithAuth(url: string, options: RequestInit = {}): Promise<Response> {
    const token = this.getToken();
    
    if (!token) {
      throw new Error("No authentication token");
    }

    const headers = {
      ...options.headers,
      "Authorization": `Bearer ${token}`,
    };

    const response = await fetch(url, {
      ...options,
      headers,
    });

    if (response.status === 401) {
      // Token is invalid, clear local storage
      this.clear();
      throw new Error("Authentication expired");
    }

    return response;
  }
}