import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("sqlite:bbq_system.db");
}

export interface PosItem {
  prep_item_id: number;
  raw_item_id: number;
  pos_display_name: string;
  current_stock_pieces: number;
  unit_price: number;
  is_variable_price: number; // NEW: 0 for Fixed, 1 for Variable
}

export interface CartItem extends PosItem {
  qty: number;
}

export const posService = {
  
  async getAvailablePosItems(): Promise<PosItem[]> {
    const db = await getDb();
    return await db.select<PosItem[]>(
      "SELECT * FROM Prepared_Inventory WHERE current_stock_pieces > 0 ORDER BY pos_display_name ASC"
    );
  },

  async processCheckout(staffId: number, cartItems: CartItem[], subtotal: number, tax: number, total: number): Promise<number> {
    const db = await getDb();
    
    await db.execute("BEGIN TRANSACTION");
    
    try {
      const orderResult = await db.execute(
        "INSERT INTO Orders (staff_id, subtotal, tax_amount, total_amount) VALUES ($1, $2, $3, $4)",
        [staffId, subtotal, tax, total]
      );
      
      const orderId = orderResult.lastInsertId as number;

      for (const item of cartItems) {
        // Saves the exact price at the time of sale (handles variable pricing perfectly)
        await db.execute(
          "INSERT INTO Order_Item (order_id, prep_item_id, quantity, price_at_time_of_sale) VALUES ($1, $2, $3, $4)",
          [orderId, item.prep_item_id, item.qty, item.unit_price]
        );

        await db.execute(
          "UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces - $1 WHERE prep_item_id = $2",
          [item.qty, item.prep_item_id]
        );
      }

      const totalItems = cartItems.reduce((sum, item) => sum + item.qty, 0);
      await db.execute(
        "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES ($1, $2, $3, $4, $5)",
        [`POS-${orderId}`, 'POS', staffId, `Ticket #${orderId} Processed`, `₱${total.toFixed(2)} (${totalItems} items)`]
      );

      await db.execute("COMMIT");
      return orderId;
      
    } catch (error) {
      await db.execute("ROLLBACK");
      throw error;
    }
  }
};