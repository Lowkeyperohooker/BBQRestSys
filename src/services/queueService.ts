import { type CartItem } from './posService';

const hostname = window.location.hostname === 'tauri.localhost' 
  ? '127.0.0.1' 
  : window.location.hostname;

const API_BASE_URL = `http://${hostname}:3000/api`;

export interface PendingOrder {
  queue_number: number;
  order_type: string;
  cart_items: CartItem[];
  subtotal: number;
  tax: number;
  total: number;
  timestamp: string;
}

export const queueService = {
  // Cashier uses this to pull pending Kiosk orders
  async getQueue(): Promise<PendingOrder[]> {
    const res = await fetch(`${API_BASE_URL}/queue`);
    if (!res.ok) throw new Error('Failed to fetch queue');
    return await res.json();
  },

  // Kiosk uses this to push a new order to the JSON file
  async addToQueue(order: PendingOrder): Promise<void> {
    const res = await fetch(`${API_BASE_URL}/queue/add`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(order)
    });
    if (!res.ok) throw new Error('Failed to add to queue');
  },

  // Cashier uses this to delete the order from JSON once they accept it into the database
  async removeFromQueue(queueNumber: number): Promise<void> {
    const res = await fetch(`${API_BASE_URL}/queue/remove/${queueNumber}`, {
      method: 'POST'
    });
    if (!res.ok) throw new Error('Failed to remove from queue');
  },

  // Fetch the strict, permanent counter from the Rust server
  async getNextQueueNumber(): Promise<number> {
    const res = await fetch(`${API_BASE_URL}/queue/next-number`);
    if (!res.ok) throw new Error('Failed to get next queue number');
    return await res.json();
  }
};