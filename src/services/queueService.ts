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

  // NEW: Safely increment the queue number from 1000 to 9090
  async getNextQueueNumber(): Promise<number> {
    const lastQueueStr = localStorage.getItem('last_queue_number');
    let queueNum = lastQueueStr ? parseInt(lastQueueStr) + 1 : 1000;

    // Enforce the range limits
    if (queueNum > 9090) {
      queueNum = 1000;
    }

    // Persist the new number for the next customer
    localStorage.setItem('last_queue_number', queueNum.toString());
    return queueNum;
  }
};