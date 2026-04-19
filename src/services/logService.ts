import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("postgres://postgres:nigmagalaxy@localhost:5432/bbq_system");
}

export interface SystemLog {
  log_id: string;
  log_category: string;
  staff_id: number;
  staff_name: string;
  timestamp: string;
  description: string;
  details: string | null;
}

export const logService = {
  // Fetch logs, ordered from newest to oldest
  async getRecentLogs(limit: number = 100): Promise<SystemLog[]> {
    const db = await getDb();
    
    // We use a LEFT JOIN to attach the staff member's actual name to the log record
    const query = `
      SELECT 
        sl.log_id, 
        sl.log_category, 
        sl.staff_id, 
        s.full_name as staff_name, 
        sl.timestamp, 
        sl.description, 
        sl.details 
      FROM System_Log sl
      LEFT JOIN Staff s ON sl.staff_id = s.staff_id
      ORDER BY sl.timestamp DESC
      LIMIT $1
    `;
    
    return await db.select<SystemLog[]>(query, [limit]);
  }
};