const hostname = window.location.hostname === 'tauri.localhost' 
  ? '127.0.0.1' 
  : window.location.hostname;

const API_BASE_URL = `http://${hostname}:3000/api`;

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