import Database from "@tauri-apps/plugin-sql";

async function getDb() {
  return await Database.load("postgres://postgres:nigmagalaxy@localhost:5432/bbq_system"); 
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

// Helper function to fetch the itemized receipt from the database for existing orders
async function getOrderReceiptString(db: Database, orderId: number): Promise<string> {
  const items = await db.select<{pos_display_name: string, quantity: number, price_at_time_of_sale: number}[]>(`
    SELECT pi.pos_display_name, oi.quantity, oi.price_at_time_of_sale 
    FROM Order_Item oi
    JOIN Prepared_Inventory pi ON oi.prep_item_id = pi.prep_item_id
    WHERE oi.order_id = $1
  `, [orderId]);

  if (items.length === 0) return "No items found.";

  return items.map(item => 
    `- ${item.quantity}x ${item.pos_display_name} (PHP ${(item.price_at_time_of_sale * item.quantity).toFixed(2)})`
  ).join('\n');
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
    // We filter out 'Pending PIN' so they don't show up on the grill screen until paid/activated!
    return await db.select<ActiveOrder[]>(
      "SELECT order_id, customer_identifier, order_type, total_amount, status, timestamp FROM Orders WHERE status != 'Completed' AND status != 'Pending PIN' ORDER BY timestamp ASC"
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

      // Generate the initial detailed receipt for the log
      const itemizedList = cartItems.map(item => 
        `- ${item.qty}x ${item.pos_display_name} (PHP ${(item.unit_price * item.qty).toFixed(2)})`
      ).join('\n');

      const detailedLogMessage = `Order Type: ${orderType}\nCustomer/Table: ${customerIdentifier}\n\nItems Ordered:\n${itemizedList}\n\nTotal Amount: PHP ${total.toFixed(2)}`;

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

  // ==========================================
  // KIOSK PIN FUNCTIONS
  // ==========================================

  // 1. Generate a PIN and hold the order (Client/Touchpad action)
  async holdOrderWithPin(
    staffId: number, 
    customerIdentifier: string, 
    orderType: string, 
    cartItems: CartItem[], 
    subtotal: number, 
    tax: number, 
    total: number
  ): Promise<string> {
    const db = await getDb();
    
    // Generate a random 4-digit PIN (1000 - 9999)
    const pin = Math.floor(1000 + Math.random() * 9000).toString();
    
    await db.execute("BEGIN TRANSACTION");
    try {
      // Save order with status 'Pending PIN'
      const orderResult = await db.execute(
        "INSERT INTO Orders (staff_id, customer_identifier, order_type, pin, subtotal, tax_amount, total_amount, status) VALUES ($1, $2, $3, $4, $5, $6, $7, 'Pending PIN')",
        [staffId, customerIdentifier, orderType, pin, subtotal, tax, total]
      );
      
      const orderId = orderResult.lastInsertId as number;

      for (const item of cartItems) {
        await db.execute(
          "INSERT INTO Order_Item (order_id, prep_item_id, quantity, price_at_time_of_sale) VALUES ($1, $2, $3, $4)",
          [orderId, item.prep_item_id, item.qty, item.unit_price]
        );
        // NOTE: We do NOT deduct inventory yet, because they haven't paid!
      }

      await db.execute("COMMIT");
      return pin; // Return the PIN to show on the client screen
    } catch (error) {
      await db.execute("ROLLBACK");
      throw error;
    }
  },

  // 2. Cashier enters PIN to retrieve and activate the order
  async activateOrderByPin(pin: string, staffId: number): Promise<ActiveOrder | null> {
    const db = await getDb();
    
    // Find the order
    const orders = await db.select<ActiveOrder[]>(
      "SELECT * FROM Orders WHERE pin = $1 AND status = 'Pending PIN'",
      [pin]
    );

    if (orders.length === 0) return null; // Invalid PIN
    
    const order = orders[0];

    // Change status to Cooking, clear the PIN so it can't be used again
    await db.execute(
      "UPDATE Orders SET status = 'Cooking', pin = NULL WHERE order_id = $1",
      [order.order_id]
    );

    // Now that it is confirmed, deduct the inventory items
    const items = await db.select<{prep_item_id: number, quantity: number}[]>(
      "SELECT prep_item_id, quantity FROM Order_Item WHERE order_id = $1",
      [order.order_id]
    );

    for (const item of items) {
      await db.execute(
        "UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces - $1 WHERE prep_item_id = $2",
        [item.quantity, item.prep_item_id]
      );
    }

    const receipt = await getOrderReceiptString(db, order.order_id);

    // Log the action
    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'POS', $1, 'PIN Order Activated', $2)",
      [staffId, `Order #${order.order_id} activated via PIN ${pin}. Sent to Grill.\n\nOrder Contents:\n${receipt}`]
    );

    return order;
  },

  // ==========================================

  async updateOrderStatus(orderId: number, status: string, staffId: number): Promise<void> {
    const db = await getDb();
    
    // Fetch the detailed receipt before logging
    const receipt = await getOrderReceiptString(db, orderId);

    await db.execute(
      "UPDATE Orders SET status = $1 WHERE order_id = $2",
      [status, orderId]
    );

    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'POS', $1, 'Order Status Updated', $2)",
      [staffId, `Order #${orderId} marked as ${status}\n\nOrder Contents:\n${receipt}`]
    );
  },

  async settlePayment(orderId: number, staffId: number): Promise<void> {
    const db = await getDb();
    
    // Fetch the detailed receipt before logging
    const receipt = await getOrderReceiptString(db, orderId);
    
    await db.execute(
      "UPDATE Orders SET status = 'Completed' WHERE order_id = $1",
      [orderId]
    );

    await db.execute(
      "INSERT INTO System_Log (log_id, log_category, staff_id, description, details) VALUES (hex(randomblob(8)), 'POS', $1, 'Payment Settled', $2)",
      [staffId, `Order #${orderId} has been paid in full.\n\nOrder Contents:\n${receipt}`]
    );
  }
};