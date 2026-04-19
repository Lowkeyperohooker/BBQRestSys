import { invoke } from '@tauri-apps/api/core';

export interface SystemLog {
  log_id: string;
  log_category: string;
  staff_id: number;
  staff_name: string | null;
  timestamp: string;
  description: string;
  details: string | null;
}

export const logService = {
  async getRecentLogs(limit: number = 100): Promise<SystemLog[]> {
    return await invoke('get_recent_logs', { limit });
  },
};