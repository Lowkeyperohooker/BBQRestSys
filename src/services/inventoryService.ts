import { invoke } from '@tauri-apps/api/core';

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

const CURRENT_ADMIN_ID = 1;

export const inventoryService = {
  async getRawInventory(): Promise<RawInventoryItem[]> {
    return await invoke('get_raw_inventory');
  },
  async getPreparedInventory(): Promise<PreparedInventoryItem[]> {
    return await invoke('get_prepared_inventory');
  },
  async addRawStock(itemId: number, kilosToAdd: number): Promise<void> {
    await invoke('add_raw_stock', { itemId, kilosToAdd, staffId: CURRENT_ADMIN_ID });
  },
  async addNewRawItem(category: string, part: string, initialKilos: number, alertThreshold: number): Promise<void> {
    await invoke('add_new_raw_item', { category, part, initialKilos, alertThreshold, staffId: CURRENT_ADMIN_ID });
  },
  async updatePreparedItemPricing(prepItemId: number, newPrice: number, isVariable: boolean): Promise<void> {
    await invoke('update_prepared_item_pricing', { prepItemId, newPrice, isVariable, staffId: CURRENT_ADMIN_ID });
  },
  async getAvailableCategories(): Promise<string[]> {
    return await invoke('get_available_categories');
  },
  async getAvailableParts(category: string): Promise<RawInventoryItem[]> {
    return await invoke('get_available_parts', { category });
  },
  async checkStockAvailability(category: string, part: string, kilosNeeded: number): Promise<{ available: boolean; currentStock: number }> {
    const parts: RawInventoryItem[] = await invoke('get_available_parts', { category });
    const item = parts.find(p => p.specific_part === part);
    if (!item) return { available: false, currentStock: 0 };
    return { available: item.current_stock_kg >= kilosNeeded, currentStock: item.current_stock_kg };
  },
  async logPrepTransaction(category: string, part: string, kilos: number, sticks: number, staffName?: string): Promise<void> {
    await invoke('log_prep_transaction', { category, part, kilos, sticks, staffName: staffName ?? null });
  },
  async getLowStockRawItems(): Promise<RawInventoryItem[]> {
    return await invoke('get_low_stock_alerts');
  },
  async getRecentPrepLogs(limit: number = 10): Promise<PrepLog[]> {
    return await invoke('get_recent_prep_logs', { limit });
  },
};