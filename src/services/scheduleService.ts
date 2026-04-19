import { invoke } from '@tauri-apps/api/core';

export interface Shift {
  shift_id: number;
  staff_id: number;
  full_name: string;
  role: string;
  shift_date: string;
  clock_in_time: string;
  clock_out_time: string | null;
  total_rendered_hours: number | null;
  status: string | null;
}

export const scheduleService = {
  async getTodayShifts(): Promise<Shift[]> {
    return await invoke('get_today_shifts');
  },
  async getActiveShiftForStaff(staffId: number): Promise<Shift | null> {
    return await invoke('get_active_shift_for_staff', { staffId });
  },
  async clockIn(staffId: number): Promise<void> {
    await invoke('clock_in', { staffId });
  },
  async clockOut(shiftId: number, staffId: number): Promise<void> {
    await invoke('clock_out', { shiftId, staffId });
  },
};