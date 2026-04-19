const API_BASE = 'http://localhost:3000/api';

export const dashboardService = {
  async getTodaySales(): Promise<number> {
    const res = await fetch(`${API_BASE}/dashboard/sales`);
    if (!res.ok) throw new Error('Failed to fetch sales');
    return await res.json();
  },
  
  async getActiveStaffCount(): Promise<number> {
    const res = await fetch(`${API_BASE}/dashboard/staff-count`);
    if (!res.ok) throw new Error('Failed to fetch staff count');
    return await res.json();
  },
  
  async getLowStockAlerts(): Promise<any[]> {
    const res = await fetch(`${API_BASE}/dashboard/low-stock`);
    if (!res.ok) throw new Error('Failed to fetch stock alerts');
    return await res.json();
  },
  
  async getTopSellingItems(): Promise<any[]> {
    const res = await fetch(`${API_BASE}/dashboard/top-items`);
    if (!res.ok) throw new Error('Failed to fetch top items');
    return await res.json();
  },
};