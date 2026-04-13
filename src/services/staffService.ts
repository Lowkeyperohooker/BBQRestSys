import Database from "@tauri-apps/plugin-sql";

// Step 1.6: Setup the connection to our specific SQLite file
async function getDb() {
  return await Database.load("sqlite:bbq_system.db");
}

export const staffService = {
  
  // Step 1.7: Fetch all staff members, ordered by active status then by name
  async getAllStaff() {
    const db = await getDb();
    return await db.select("SELECT * FROM Staff ORDER BY status ASC, full_name ASC");
  },

  // Step 1.8: Insert a brand new staff member into the database
  async createStaff(staff: { name: string; role: string; phone: string; status: string }) {
    const db = await getDb();
    await db.execute(
      "INSERT INTO Staff (full_name, role, phone_number, status) VALUES ($1, $2, $3, $4)",
      [staff.name, staff.role, staff.phone, staff.status]
    );
  },

  // Step 1.9: Update an existing staff member's details
  async updateStaff(id: number, staff: { name: string; role: string; phone: string; status: string }) {
    const db = await getDb();
    await db.execute(
      "UPDATE Staff SET full_name = $1, role = $2, phone_number = $3, status = $4 WHERE staff_id = $5",
      [staff.name, staff.role, staff.phone, staff.status, id]
    );
  }
  
};