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
  is_variable_price: number;
}

export interface CartItem extends PosItem {
  qty: number;
}

export interface ActiveOrder {
  order_id: number;
  customer_identifier: string;
  order_type: string;
  total_amount: number;
  status: string;
  timestamp: string;
}

export const posService = {
  
  async getAvailablePosItems(): Promise<PosItem[]> {
    const db = await getDb();
    return await db.select<PosItem[]>(
      "SELECT * FROM Prepared_Inventory WHERE current_stock_pieces > 0 ORDER BY pos_display_name ASC"
    );
  },

  async getActiveOrders(): Promise<ActiveOrder[]> {
    const db = await getDb();
    return await db.select<ActiveOrder[]>(
      "SELECT order_id, customer_identifier, order_type, total_amount, status, timestamp FROM Orders WHERE status != 'Completed' ORDER BY timestamp ASC"
    );
  },

  async sendToGrill(
    staffId: number, 
    customerIdentifier: string, 
    orderType: string, 
    cartItems: CartItem[], 
    subtotal: number, 
    tax: number, 
    total: number
  ): Promise<number> {
    const db = await getDb();
    await db.execute("BEGIN TRANSACTION");
    
    try {
      const orderResult = await db.execute(
        "INSERT INTO Orders (staff_id, customer_identifier, order_type, subtotal, tax_amount, total_amount, status) VALUES ($1, $2, $3, $4, $5, $6, 'Cooking')",
        [staffId, customerIdentifier, orderType, subtotal, tax, total]
      );
      
      const orderId = orderResult.lastInsertId as number;

      for (const item of cartItems) {
        await db.execute(
          "INSERT INTO Order_Item (order_id, prep_item_id, quantity, price_at_time_of_sale) VALUES ($1, $2, $3, $4)",
          [orderId, item.prep_item_id, item.qty, item.unit_price]
        );

        await db.execute(
          "UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces - $1 WHERE prep_item_id = $2",
          [item.qty, item.prep_item_id]
        );
      }

      // ==========================================
      // NEW: Generate a detailed, itemized receipt string for the log
      // ==========================================
      const itemizedList = cartItems.map(item => 
        `- ${item.qty}x ${item.pos_display_name} (₱${(item.unit_price * item.qty).toFixed(2)})`
      ).join('\n');

      const detailedLogMessage = `Order Type: ${orderType}\nCustomer/Table: ${customerIdentifier}\n\nItems Ordered:\n${itemizedList}\n\nTotal Amount: ₱${total.toFixed(2)}`;

      await db.execute(
        "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'POS', $1, 'Order Sent to Grill', $2)",
        [staffId, detailedLogMessage]
      );

      await db.execute("COMMIT");
      return orderId;
      
    } catch (error) {
      await db.execute("ROLLBACK");
      throw error;
    }
  },

  async updateOrderStatus(orderId: number, status: string, staffId: number): Promise<void> {
    const db = await getDb();
    await db.execute(
      "UPDATE Orders SET status = $1 WHERE order_id = $2",
      [status, orderId]
    );

    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'POS', $1, 'Order Status Updated', $2)",
      [staffId, `Order #${orderId} marked as ${status}`]
    );
  },

  async settlePayment(orderId: number, staffId: number): Promise<void> {
    const db = await getDb();
    
    await db.execute(
      "UPDATE Orders SET status = 'Completed' WHERE order_id = $1",
      [orderId]
    );

    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'POS', $1, 'Payment Settled', $2)",
      [staffId, `Order #${orderId} has been paid in full.`]
    );
  }
};