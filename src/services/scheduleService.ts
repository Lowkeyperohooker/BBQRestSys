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

const API_BASE = window.location.hostname === 'localhost' 
  ? 'http://localhost:3000/api' 
  : `http://${window.location.hostname}:3000/api`;

export const scheduleService = {
  async getTodayShifts(): Promise<Shift[]> {
    const res = await fetch(`${API_BASE}/schedule/today`);
    if (!res.ok) throw new Error('Failed to fetch today shifts');
    return await res.json();
  },
  
  async getActiveShiftForStaff(staffId: number): Promise<Shift | null> {
    const res = await fetch(`${API_BASE}/schedule/active-shift?staff_id=${staffId}`);
    if (!res.ok) throw new Error('Failed to fetch active shift');
    return await res.json();
  },
  
  async clockIn(staffId: number | string): Promise<void> {
    const res = await fetch(`${API_BASE}/schedule/clock-in`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ staff_id: Number(staffId) }) // Force it to be a number
    });
    
    if (!res.ok) {
      // Extract the actual error message sent by Rust
      const errorText = await res.text();
      throw new Error(errorText || 'Failed to clock in');
    }
  },
  
  async clockOut(shiftId: number | string, staffId: number | string): Promise<void> {
    const res = await fetch(`${API_BASE}/schedule/clock-out`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ shift_id: Number(shiftId), staff_id: Number(staffId) }) // Force numbers
    });
    
    if (!res.ok) {
      const errorText = await res.text();
      throw new Error(errorText || 'Failed to clock out');
    }
  },
};