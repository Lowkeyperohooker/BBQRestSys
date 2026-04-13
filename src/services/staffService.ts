import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("sqlite:bbq_system.db");
}

export interface StaffMember {
  staff_id: number;
  full_name: string;
  role: string;
  phone_number: string | null;
  status: 'Active' | 'Inactive';
  created_at: string;
}

export interface StaffInput {
  name: string;
  role: string;
  phone: string;
  status: string;
}

export const staffService = {
  
  async getAllStaff(): Promise<StaffMember[]> {
    const db = await getDb();
    return await db.select<StaffMember[]>(
      "SELECT * FROM Staff ORDER BY status ASC, full_name ASC"
    );
  },

  async createStaff(staff: StaffInput): Promise<void> {
    const db = await getDb();
    await db.execute(
      "INSERT INTO Staff (full_name, role, phone_number, status) VALUES ($1, $2, $3, $4)",
      [staff.name, staff.role, staff.phone, staff.status]
    );
  },

  async updateStaff(id: number, staff: StaffInput): Promise<void> {
    const db = await getDb();
    await db.execute(
      "UPDATE Staff SET full_name = $1, role = $2, phone_number = $3, status = $4 WHERE staff_id = $5",
      [staff.name, staff.role, staff.phone, staff.status, id]
    );
  },

  async deleteStaff(id: number): Promise<void> {
    const db = await getDb();
    await db.execute("DELETE FROM Staff WHERE staff_id = $1", [id]);
  },
  
  // Optional: Get active staff only
  async getActiveStaff(): Promise<StaffMember[]> {
    const db = await getDb();
    return await db.select<StaffMember[]>(
      "SELECT * FROM Staff WHERE status = 'Active' ORDER BY full_name ASC"
    );
  },
  
  // Optional: Search staff by name or role
  async searchStaff(query: string): Promise<StaffMember[]> {
    const db = await getDb();
    return await db.select<StaffMember[]>(
      "SELECT * FROM Staff WHERE full_name LIKE $1 OR role LIKE $1 ORDER BY full_name ASC",
      [`%${query}%`]
    );
  }
};