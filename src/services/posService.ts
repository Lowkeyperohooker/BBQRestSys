import { inventoryService } from './inventoryService';

export interface PosItem {
  prep_item_id: number;
  raw_item_id: number;
  pos_display_name: string;
  current_stock_pieces: number;
  unit_price: number;
  is_variable_price: boolean;
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

const API_BASE = 'http://localhost:3000/api';

export const posService = {
  async getAvailablePosItems(): Promise<PosItem[]> {
    const items = await inventoryService.getPreparedInventory();
    return items.filter((i: any) => i.current_stock_pieces > 0);
  },
  
  async getActiveOrders(): Promise<ActiveOrder[]> {
    const res = await fetch(`${API_BASE}/pos/active-orders`);
    if (!res.ok) throw new Error('Failed to fetch active orders');
    return await res.json();
  },
  
  async sendToGrill(
    staffId: number,
    customerIdentifier: string,
    orderType: string,
    cartItems: CartItem[],
    subtotal: number,
    tax: number,
    total: number,
  ): Promise<number> {
    const formattedCart = cartItems.map(i => ({
      prep_item_id: i.prep_item_id,
      qty: i.qty,
      unit_price: i.unit_price,
      pos_display_name: i.pos_display_name,
    }));

    const res = await fetch(`${API_BASE}/pos/send-to-grill`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        staff_id: staffId, 
        customer_identifier: customerIdentifier, 
        order_type: orderType,
        cart_items: formattedCart,
        subtotal, 
        tax, 
        total
      })
    });
    
    if (!res.ok) throw new Error('Failed to send order to grill');
    return await res.json();
  },
  
  async updateOrderStatus(orderId: number, status: string, staffId: number): Promise<void> {
    const res = await fetch(`${API_BASE}/pos/update-status`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ order_id: orderId, status, staff_id: staffId })
    });
    if (!res.ok) throw new Error('Failed to update order status');
  },
  
  async settlePayment(orderId: number, staffId: number): Promise<void> {
    const res = await fetch(`${API_BASE}/pos/settle`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ order_id: orderId, staff_id: staffId })
    });
    if (!res.ok) throw new Error('Failed to settle payment');
  },
};