import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("postgres://postgres:nigmagalaxy@localhost:5432/bbq_system");
}

export const dashboardService = {
  // 1. Get Total Sales for Today
  async getTodaySales(): Promise<number> {
    const db = await getDb();
    // SQLite uses 'now' and 'localtime' to match the computer's current day
    const result = await db.select<any[]>(
      "SELECT SUM(total_amount) as total FROM Orders WHERE date(timestamp) = date('now', 'localtime')"
    );
    return result[0]?.total || 0;
  },

  // 2. Count Active Staff Members
  async getActiveStaffCount(): Promise<number> {
    const db = await getDb();
    const result = await db.select<any[]>(
      "SELECT COUNT(*) as count FROM Staff WHERE status = 'Active'"
    );
    return result[0]?.count || 0;
  },

  // 3. Get Low Stock Alerts (Raw Inventory)
  async getLowStockAlerts(): Promise<any[]> {
    const db = await getDb();
    return await db.select<any[]>(
      "SELECT category, specific_part, current_stock_kg, alert_threshold_kg FROM Raw_Inventory WHERE current_stock_kg <= alert_threshold_kg"
    );
  },

  // 4. Get Top Selling Items (Prepared Inventory)
  async getTopSellingItems(): Promise<any[]> {
    const db = await getDb();
    return await db.select<any[]>(
      `SELECT 
        pi.pos_display_name, 
        SUM(oi.quantity) as total_sold 
       FROM Order_Item oi
       JOIN Prepared_Inventory pi ON oi.prep_item_id = pi.prep_item_id
       GROUP BY oi.prep_item_id
       ORDER BY total_sold DESC
       LIMIT 5`
    );
  }
};