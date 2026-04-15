import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("sqlite:bbq_system.db");
}

export interface RawInventoryItem {
  raw_item_id: number;
  category: string;
  specific_part: string;
  current_stock_kg: number;
  alert_threshold_kg: number;
}

export interface PreparedInventoryItem {
  prep_item_id: number;
  raw_item_id: number;
  pos_display_name: string;
  current_stock_pieces: number;
  unit_price: number;
}

export interface PrepLog {
  timestamp: string;
  staff_name: string;
  category: string;
  specific_part: string;
  kilos_deducted: number;
  skewers_added: number;
}

// Hardcoded Admin ID for actions where a specific staff isn't selected
const CURRENT_ADMIN_ID = 1;

export const inventoryService = {
  
  async getRawInventory(): Promise<RawInventoryItem[]> {
    const db = await getDb();
    return await db.select<RawInventoryItem[]>(
      "SELECT * FROM Raw_Inventory ORDER BY category ASC, specific_part ASC"
    );
  },

  async getPreparedInventory(): Promise<PreparedInventoryItem[]> {
    const db = await getDb();
    return await db.select<PreparedInventoryItem[]>(
      "SELECT * FROM Prepared_Inventory ORDER BY pos_display_name ASC"
    );
  },
  
  async addRawStock(itemId: number, kilosToAdd: number): Promise<void> {
    const db = await getDb();
    await db.execute(
      "UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg + $1 WHERE raw_item_id = $2",
      [kilosToAdd, itemId]
    );

    // NEW: Connect to System Logs
    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'INVENTORY', $1, 'Stock Delivery Added', $2)",
      [CURRENT_ADMIN_ID, `Added ${kilosToAdd}kg to Item ID: ${itemId}`]
    );
  },
  
  async getAvailableCategories(): Promise<string[]> {
    const db = await getDb();
    const result = await db.select<{category: string}[]>(
      "SELECT DISTINCT category FROM Raw_Inventory WHERE current_stock_kg > 0 ORDER BY category"
    );
    return result.map(r => r.category);
  },
  
  async getAvailableParts(category: string): Promise<RawInventoryItem[]> {
    const db = await getDb();
    return await db.select<RawInventoryItem[]>(
      "SELECT * FROM Raw_Inventory WHERE category = $1 AND current_stock_kg > 0 ORDER BY specific_part",
      [category]
    );
  },
  
  async checkStockAvailability(category: string, part: string, kilosNeeded: number): Promise<{available: boolean, currentStock: number}> {
    const db = await getDb();
    const result = await db.select<{current_stock_kg: number}[]>(
      "SELECT current_stock_kg FROM Raw_Inventory WHERE category = $1 AND specific_part = $2",
      [category, part]
    );
    
    if (result.length === 0) {
      return { available: false, currentStock: 0 };
    }
    
    const currentStock = result[0].current_stock_kg;
    return {
      available: currentStock >= kilosNeeded,
      currentStock
    };
  },
  
  async logPrepTransaction(
    category: string, 
    part: string, 
    kilos: number, 
    sticks: number,
    staffName?: string
  ) {
    const db = await getDb();
    
    // Check stock availability first
    const stockCheck = await this.checkStockAvailability(category, part, kilos);
    if (!stockCheck.available) {
      throw new Error(`Insufficient stock! Available: ${stockCheck.currentStock}kg, Needed: ${kilos}kg`);
    }
    
    await db.execute("BEGIN TRANSACTION");
    
    try {
      // First, get the raw_item_id
      const rawItems = await db.select<{raw_item_id: number}[]>(
        "SELECT raw_item_id FROM Raw_Inventory WHERE category = $1 AND specific_part = $2",
        [category, part]
      );
      
      if (rawItems.length === 0) {
        throw new Error(`Raw item not found: ${category} - ${part}`);
      }
      
      const rawItemId = rawItems[0].raw_item_id;
      
      // Deduct raw kilos
      await db.execute(
        "UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg - $1 WHERE raw_item_id = $2",
        [kilos, rawItemId]
      );

      // Add prepared sticks
      await db.execute(
        "UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces + $1 WHERE raw_item_id = $2",
        [sticks, rawItemId]
      );
      
      let actualStaffId = CURRENT_ADMIN_ID;

      // Get staff_id if staffName provided
      if (staffName) {
        const staff = await db.select<{staff_id: number}[]>(
          "SELECT staff_id FROM Staff WHERE full_name = $1",
          [staffName]
        );
        
        if (staff.length > 0) {
          actualStaffId = staff[0].staff_id;
          // Log the prep activity
          await db.execute(
            `INSERT INTO Prep_Log (staff_id, raw_item_id, kilos_deducted, skewers_added) 
             VALUES ($1, $2, $3, $4)`,
            [actualStaffId, rawItemId, kilos, sticks]
          );
        }
      }
      
      // NEW: Connect to System Logs
      await db.execute(
        "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'PREP', $1, 'Skewers Prepared', $2)",
        [actualStaffId, `${staffName || 'System'} converted ${kilos}kg into ${sticks} sticks`]
      );
      
      await db.execute("COMMIT");
    } catch (error) {
      await db.execute("ROLLBACK");
      throw error;
    }
  },
  
  async getLowStockRawItems(): Promise<RawInventoryItem[]> {
    const db = await getDb();
    return await db.select<RawInventoryItem[]>(
      "SELECT * FROM Raw_Inventory WHERE current_stock_kg <= alert_threshold_kg"
    );
  },
  
  async getRecentPrepLogs(limit: number = 10): Promise<PrepLog[]> {
    const db = await getDb();
    return await db.select<PrepLog[]>(`
      SELECT 
        p.timestamp,
        s.full_name as staff_name,
        r.category,
        r.specific_part,
        p.kilos_deducted,
        p.skewers_added
      FROM Prep_Log p
      JOIN Staff s ON p.staff_id = s.staff_id
      JOIN Raw_Inventory r ON p.raw_item_id = r.raw_item_id
      ORDER BY p.timestamp DESC
      LIMIT $1
    `, [limit]);
  }
};