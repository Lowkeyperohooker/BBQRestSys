import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("postgres://postgres:nigmagalaxy@localhost:5432/bbq_system");
}

export interface Shift {
  shift_id: number;
  staff_id: number;
  full_name: string;
  role: string;
  shift_date: string;
  clock_in_time: string;
  clock_out_time: string | null;
  total_rendered_hours: number | null;
  status: string;
}

export const scheduleService = {
  // Get all shifts for today (joins with Staff table to get names)
  async getTodayShifts(): Promise<Shift[]> {
    const db = await getDb();
    const query = `
      SELECT 
        sh.shift_id, sh.staff_id, s.full_name, s.role, 
        sh.shift_date, sh.clock_in_time, sh.clock_out_time, 
        sh.total_rendered_hours, sh.status
      FROM Shift sh
      JOIN Staff s ON sh.staff_id = s.staff_id
      WHERE sh.shift_date = date('now', 'localtime')
      ORDER BY sh.clock_in_time DESC
    `;
    return await db.select<Shift[]>(query);
  },

  // Check if a specific staff member is already clocked in right now
  async getActiveShiftForStaff(staffId: number): Promise<Shift | null> {
    const db = await getDb();
    const result = await db.select<Shift[]>(
      "SELECT * FROM Shift WHERE staff_id = $1 AND status = 'Active Shift'",
      [staffId]
    );
    return result.length > 0 ? result[0] : null;
  },

  // Clock In
  async clockIn(staffId: number): Promise<void> {
    const db = await getDb();
    // Verify they aren't already clocked in
    const active = await this.getActiveShiftForStaff(staffId);
    if (active) throw new Error("Staff member is already clocked in!");

    const query = `
      INSERT INTO Shift (staff_id, shift_date, clock_in_time, status)
      VALUES ($1, date('now', 'localtime'), datetime('now', 'localtime'), 'Active Shift')
    `;
    await db.execute(query, [staffId]);
    
    // Log it for the admin
    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description) VALUES (hex(randomblob(8)), 'TIME', $1, 'Clocked In')",
      [staffId]
    );
  },

  // Clock Out (Calculates total hours!)
  async clockOut(shiftId: number, staffId: number): Promise<void> {
    const db = await getDb();
    
    // 1. Update the record with the clock-out time and mark as Completed
    await db.execute(
      "UPDATE Shift SET clock_out_time = datetime('now', 'localtime'), status = 'Completed' WHERE shift_id = $1",
      [shiftId]
    );

    // 2. Calculate the total hours using SQLite time math
    await db.execute(`
      UPDATE Shift 
      SET total_rendered_hours = ROUND((julianday(clock_out_time) - julianday(clock_in_time)) * 24, 2)
      WHERE shift_id = $1
    `, [shiftId]);

    // 3. Log it
    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description) VALUES (hex(randomblob(8)), 'TIME', $1, 'Clocked Out')",
      [staffId]
    );
  }
};