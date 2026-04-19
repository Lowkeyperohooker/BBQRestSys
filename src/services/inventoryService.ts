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
  is_variable_price: boolean;
}

export interface PrepLog {
  timestamp: string;
  staff_name: string;
  category: string;
  specific_part: string;
  kilos_deducted: number;
  skewers_added: number;
}

const API_BASE = 'http://localhost:3000/api';
const CURRENT_ADMIN_ID = 1;

export const inventoryService = {
  async getRawInventory(): Promise<RawInventoryItem[]> {
    const res = await fetch(`${API_BASE}/inventory/raw`);
    if (!res.ok) throw new Error('Failed to fetch raw inventory');
    return await res.json();
  },
  
  async getPreparedInventory(): Promise<PreparedInventoryItem[]> {
    const res = await fetch(`${API_BASE}/inventory/prepared`);
    if (!res.ok) throw new Error('Failed to fetch prepared inventory');
    return await res.json();
  },
  
  async addRawStock(itemId: number, kilosToAdd: number): Promise<void> {
    const res = await fetch(`${API_BASE}/inventory/add-stock`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ item_id: itemId, kilos_to_add: kilosToAdd, staff_id: CURRENT_ADMIN_ID })
    });
    if (!res.ok) throw new Error('Failed to add raw stock');
  },
  
  async addNewRawItem(category: string, part: string, initialKilos: number, alertThreshold: number): Promise<void> {
    const res = await fetch(`${API_BASE}/inventory/add-new`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ category, part, initial_kilos: initialKilos, alert_threshold: alertThreshold, staff_id: CURRENT_ADMIN_ID })
    });
    if (!res.ok) throw new Error('Failed to add new raw item');
  },
  
  async updatePreparedItemPricing(prepItemId: number, newPrice: number, isVariable: boolean): Promise<void> {
    const res = await fetch(`${API_BASE}/inventory/update-pricing`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ prep_item_id: prepItemId, new_price: newPrice, is_variable: isVariable, staff_id: CURRENT_ADMIN_ID })
    });
    if (!res.ok) throw new Error('Failed to update pricing');
  },
  
  async getAvailableCategories(): Promise<string[]> {
    const res = await fetch(`${API_BASE}/inventory/categories`);
    if (!res.ok) throw new Error('Failed to fetch categories');
    return await res.json();
  },
  
  async getAvailableParts(category: string): Promise<RawInventoryItem[]> {
    const res = await fetch(`${API_BASE}/inventory/parts?category=${encodeURIComponent(category)}`);
    if (!res.ok) throw new Error('Failed to fetch parts');
    return await res.json();
  },
  
  async checkStockAvailability(category: string, part: string, kilosNeeded: number): Promise<{ available: boolean; currentStock: number }> {
    const parts = await this.getAvailableParts(category);
    const item = parts.find(p => p.specific_part === part);
    if (!item) return { available: false, currentStock: 0 };
    return { available: item.current_stock_kg >= kilosNeeded, currentStock: item.current_stock_kg };
  },
  
  async logPrepTransaction(category: string, part: string, kilos: number, sticks: number, staffName?: string): Promise<void> {
    const res = await fetch(`${API_BASE}/inventory/log-prep`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ category, part, kilos, sticks, staff_name: staffName ?? null })
    });
    if (!res.ok) throw new Error('Failed to log prep transaction');
  },
  
  async getLowStockRawItems(): Promise<RawInventoryItem[]> {
    const res = await fetch(`${API_BASE}/dashboard/low-stock`);
    if (!res.ok) throw new Error('Failed to fetch low stock alerts');
    return await res.json();
  },
  
  async getRecentPrepLogs(limit: number = 10): Promise<PrepLog[]> {
    const res = await fetch(`${API_BASE}/inventory/recent-prep?limit=${limit}`);
    if (!res.ok) throw new Error('Failed to fetch recent prep logs');
    return await res.json();
  },
};