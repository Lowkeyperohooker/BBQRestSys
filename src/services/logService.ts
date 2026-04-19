export interface SystemLog {
  log_id: string;
  log_category: string;
  staff_id: number;
  staff_name: string | null;
  timestamp: string;
  description: string;
  details: string | null;
}

const API_BASE = window.location.hostname === 'localhost' 
  ? 'http://localhost:3000/api' 
  : `http://${window.location.hostname}:3000/api`;

export const logService = {
  async getRecentLogs(limit: number = 100): Promise<SystemLog[]> {
    const res = await fetch(`${API_BASE}/logs/recent?limit=${limit}`);
    if (!res.ok) throw new Error('Failed to fetch recent logs');
    return await res.json();
  },
};