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
  is_variable_price: number;
}

export interface PrepLog {
  timestamp: string;
  staff_name: string;
  category: string;
  specific_part: string;
  kilos_deducted: number;
  skewers_added: number;
}

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
    
    // Fetch the item details first so we can log exactly what was added
    const itemResult = await db.select<{category: string, specific_part: string}[]>(
      "SELECT category, specific_part FROM Raw_Inventory WHERE raw_item_id = $1", 
      [itemId]
    );
    
    const itemName = itemResult.length > 0 ? `${itemResult[0].category} - ${itemResult[0].specific_part}` : `Item #${itemId}`;

    await db.execute(
      "UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg + $1 WHERE raw_item_id = $2",
      [kilosToAdd, itemId]
    );

    // ==========================================
    // NEW: Detailed Stock Log
    // ==========================================
    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'INVENTORY', $1, 'Stock Delivery Added', $2)",
      [CURRENT_ADMIN_ID, `Item Restocked: ${itemName}\nAmount Added: ${kilosToAdd.toFixed(2)} kg`]
    );
  },

  async addNewRawItem(category: string, part: string, initialKilos: number, alertThreshold: number): Promise<void> {
    const db = await getDb();
    await db.execute("BEGIN TRANSACTION");
    try {
      const result = await db.execute(
        "INSERT INTO Raw_Inventory (category, specific_part, current_stock_kg, alert_threshold_kg) VALUES ($1, $2, $3, $4)",
        [category, part, initialKilos, alertThreshold]
      );
      
      const newItemId = result.lastInsertId as number;

      await db.execute(
        "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'INVENTORY', $1, 'New Raw Item Added', $2)",
        [CURRENT_ADMIN_ID, `Created Item #${newItemId} (${category} - ${part})\nInitial Stock: ${initialKilos.toFixed(2)} kg\nAlert Threshold: ${alertThreshold.toFixed(2)} kg`]
      );
      await db.execute("COMMIT");
    } catch (error) {
      await db.execute("ROLLBACK");
      throw error;
    }
  },

  async updatePreparedItemPricing(prepItemId: number, newPrice: number, isVariable: number): Promise<void> {
    const db = await getDb();
    
    // Fetch item name for detailed logging
    const itemResult = await db.select<{pos_display_name: string}[]>(
      "SELECT pos_display_name FROM Prepared_Inventory WHERE prep_item_id = $1", 
      [prepItemId]
    );
    const itemName = itemResult.length > 0 ? itemResult[0].pos_display_name : `Item #${prepItemId}`;

    await db.execute(
      "UPDATE Prepared_Inventory SET unit_price = $1, is_variable_price = $2 WHERE prep_item_id = $3",
      [newPrice, isVariable, prepItemId]
    );

    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'INVENTORY', $1, 'Updated Menu Pricing', $2)",
      [CURRENT_ADMIN_ID, `Item: ${itemName}\nNew Base Price: ₱${newPrice.toFixed(2)}\nPricing Type: ${isVariable === 1 ? 'Variable (Ask Cashier)' : 'Fixed Price'}`]
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
    
    const stockCheck = await this.checkStockAvailability(category, part, kilos);
    if (!stockCheck.available) {
      throw new Error(`Insufficient stock! Available: ${stockCheck.currentStock}kg, Needed: ${kilos}kg`);
    }
    
    await db.execute("BEGIN TRANSACTION");
    
    try {
      const rawItems = await db.select<{raw_item_id: number}[]>(
        "SELECT raw_item_id FROM Raw_Inventory WHERE category = $1 AND specific_part = $2",
        [category, part]
      );
      
      if (rawItems.length === 0) {
        throw new Error(`Raw item not found: ${category} - ${part}`);
      }
      
      const rawItemId = rawItems[0].raw_item_id;
      
      await db.execute(
        "UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg - $1 WHERE raw_item_id = $2",
        [kilos, rawItemId]
      );

      await db.execute(
        "UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces + $1 WHERE raw_item_id = $2",
        [sticks, rawItemId]
      );
      
      let actualStaffId = CURRENT_ADMIN_ID;

      if (staffName) {
        const staff = await db.select<{staff_id: number}[]>(
          "SELECT staff_id FROM Staff WHERE full_name = $1",
          [staffName]
        );
        
        if (staff.length > 0) {
          actualStaffId = staff[0].staff_id;
          await db.execute(
            `INSERT INTO Prep_Log (staff_id, raw_item_id, kilos_deducted, skewers_added) 
             VALUES ($1, $2, $3, $4)`,
            [actualStaffId, rawItemId, kilos, sticks]
          );
        }
      }
      
      // ==========================================
      // NEW: Detailed Prep Log
      // ==========================================
      const detailedLogMessage = `Meat Category: ${category}\nSpecific Part / Cut: ${part}\nRaw Meat Consumed: ${kilos.toFixed(2)} kg\nYield Produced: ${sticks} pieces/sticks\nStaff Member: ${staffName || 'System Admin'}`;

      await db.execute(
        "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'PREP', $1, 'Skewers Prepared', $2)",
        [actualStaffId, detailedLogMessage]
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