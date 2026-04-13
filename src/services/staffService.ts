import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("sqlite:bbq_system.db");
}

export const staffService = {
  
  async getAllStaff() {
    const db = await getDb();
    return await db.select("SELECT * FROM Staff ORDER BY status ASC, full_name ASC");
  },

  async createStaff(staff: { name: string; role: string; phone: string; status: string }) {
    const db = await getDb();
    await db.execute(
      "INSERT INTO Staff (full_name, role, phone_number, status) VALUES ($1, $2, $3, $4)",
      [staff.name, staff.role, staff.phone, staff.status]
    );
  },

  async updateStaff(id: number, staff: { name: string; role: string; phone: string; status: string }) {
    const db = await getDb();
    await db.execute(
      "UPDATE Staff SET full_name = $1, role = $2, phone_number = $3, status = $4 WHERE staff_id = $5",
      [staff.name, staff.role, staff.phone, staff.status, id]
    );
  },

  // NEW: Delete a staff member by their ID
  async deleteStaff(id: number) {
    const db = await getDb();
    await db.execute("DELETE FROM Staff WHERE staff_id = $1", [id]);
  }
  
};