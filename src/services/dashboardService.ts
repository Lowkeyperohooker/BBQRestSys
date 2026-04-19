import { invoke } from '@tauri-apps/api/core';

export const dashboardService = {
  async getTodaySales(): Promise<number> {
    return await invoke('get_today_sales');
  },
  async getActiveStaffCount(): Promise<number> {
    return await invoke('get_active_staff_count');
  },
  async getLowStockAlerts(): Promise<any[]> {
    return await invoke('get_low_stock_alerts');
  },
  async getTopSellingItems(): Promise<any[]> {
    return await invoke('get_top_selling_items');
  },
};