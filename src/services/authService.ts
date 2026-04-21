// Adjust the base URL and fetch logic to match your other services (e.g., posService.ts)
const API_BASE_URL = 'http://localhost:8080/api'; 

export interface LoginResponse {
  staff_id: number;
  full_name: string;
  role: 'Super Admin' | 'Admin' | 'Staff' | 'Customer';
}

export const authService = {
  async login(passcode: string): Promise<LoginResponse> {
    const response = await fetch(`${API_BASE_URL}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ passcode })
    });

    if (!response.ok) {
      if (response.status === 401) throw new Error("Invalid passcode or inactive account.");
      throw new Error("Server communication failed.");
    }

    return await response.json();
  }
};