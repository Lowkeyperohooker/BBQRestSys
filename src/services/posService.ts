import { invoke } from '@tauri-apps/api/core';

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

export const posService = {
  async getAvailablePosItems(): Promise<PosItem[]> {
    const items: PosItem[] = await invoke('get_prepared_inventory');
    return items.filter(i => i.current_stock_pieces > 0);
  },
  async getActiveOrders(): Promise<ActiveOrder[]> {
    return await invoke('get_active_orders');
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
    return await invoke('send_to_grill', {
      staffId, customerIdentifier, orderType,
      cartItems: cartItems.map(i => ({
        prep_item_id: i.prep_item_id,
        qty: i.qty,
        unit_price: i.unit_price,
        pos_display_name: i.pos_display_name,
      })),
      subtotal, tax, total,
    });
  },
  async updateOrderStatus(orderId: number, status: string, staffId: number): Promise<void> {
    await invoke('update_order_status_with_log', { orderId, status, staffId });
  },
  async settlePayment(orderId: number, staffId: number): Promise<void> {
    await invoke('settle_payment', { orderId, staffId });
  },
};