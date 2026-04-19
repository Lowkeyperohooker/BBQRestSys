import { invoke } from '@tauri-apps/api/core';

export interface StaffMember {
  staff_id: number;
  full_name: string;
  role: string;
  phone_number: string | null;
  status: string | null;
  created_at: string;
}

export interface StaffInput {
  name: string;
  role: string;
  phone: string | null;
  status: string;
}

const CURRENT_ADMIN_ID = 1;

export const staffService = {
  async getAllStaff(): Promise<StaffMember[]> {
    return await invoke('get_all_staff_full');
  },
  async createStaff(staff: StaffInput): Promise<void> {
    await invoke('create_staff', { staff, adminId: CURRENT_ADMIN_ID });
  },
  async updateStaff(id: number, staff: StaffInput): Promise<void> {
    await invoke('update_staff', { id, staff, adminId: CURRENT_ADMIN_ID });
  },
  async deleteStaff(id: number): Promise<void> {
    await invoke('delete_staff', { id, adminId: CURRENT_ADMIN_ID });
  },
  async getActiveStaff(): Promise<StaffMember[]> {
    const all: StaffMember[] = await invoke('get_all_staff_full');
    return all.filter(s => s.status === 'Active');
  },
  async searchStaff(query: string): Promise<StaffMember[]> {
    return await invoke('search_staff', { query });
  },
};