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
  passcode?: string; // Added to support password changes
}

const API_BASE = window.location.hostname === 'localhost' 
  ? 'http://localhost:3000/api' 
  : `http://${window.location.hostname}:3000/api`;
const CURRENT_ADMIN_ID = 1;

export const staffService = {
  async getAllStaff(): Promise<StaffMember[]> {
    const res = await fetch(`${API_BASE}/staff/all`);
    if (!res.ok) throw new Error('Failed to fetch staff list');
    return await res.json();
  },
  
  async createStaff(staff: StaffInput): Promise<void> {
    const res = await fetch(`${API_BASE}/staff/create`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ staff, admin_id: CURRENT_ADMIN_ID })
    });
    if (!res.ok) throw new Error('Failed to create staff member');
  },
  
  async updateStaff(id: number, staff: StaffInput): Promise<void> {
    const res = await fetch(`${API_BASE}/staff/update`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ id, staff, admin_id: CURRENT_ADMIN_ID })
    });
    if (!res.ok) throw new Error('Failed to update staff member');
  },
  
  async deleteStaff(id: number): Promise<void> {
    const res = await fetch(`${API_BASE}/staff/delete`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ id, admin_id: CURRENT_ADMIN_ID })
    });
    if (!res.ok) throw new Error('Failed to delete staff member');
  },
  
  async getActiveStaff(): Promise<StaffMember[]> {
    const all = await this.getAllStaff();
    return all.filter(s => s.status === 'Active');
  },
  
  async searchStaff(query: string): Promise<StaffMember[]> {
    const res = await fetch(`${API_BASE}/staff/search?query=${encodeURIComponent(query)}`);
    if (!res.ok) throw new Error('Failed to search staff');
    return await res.json();
  },
};