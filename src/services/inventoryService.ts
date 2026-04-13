import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("sqlite:bbq_system.db");
}

export const inventoryService = {
  
  // 1. Fetch all raw meat (measured in Kilos)
  async getRawInventory() {
    const db = await getDb();
    return await db.select("SELECT * FROM Raw_Inventory ORDER BY category ASC, specific_part ASC");
  },

  // 2. Fetch all ready-to-sell skewers (measured in Pieces)
  async getPreparedInventory() {
    const db = await getDb();
    return await db.select("SELECT * FROM Prepared_Inventory ORDER BY category ASC, specific_part ASC");
  },
  
  // 3. NEW: The complex prep transaction!
  async logPrepTransaction(category: string, part: string, kilos: number, sticks: number) {
    const db = await getDb();
    
    // We deduct the raw kilos from the Raw table
    await db.execute(
      "UPDATE Raw_Inventory SET current_stock = current_stock - $1 WHERE category = $2 AND specific_part = $3",
      [kilos, category, part]
    );

    // And immediately add the sticks to the Prepared table
    await db.execute(
      "UPDATE Prepared_Inventory SET prepared_stock = prepared_stock + $1 WHERE category = $2 AND specific_part = $3",
      [sticks, category, part]
    );
  }
  
};