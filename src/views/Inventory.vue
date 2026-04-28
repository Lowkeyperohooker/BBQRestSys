<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import type { RawInventoryItem, PreparedInventoryItem, POSCategory } from '../services/inventoryService';
import { inventoryService } from '../services/inventoryService';
import EditInventoryModal from '../components/modal/EditInventoryModal.vue';
import EditPriceModal from '../components/modal/EditPriceModal.vue';
import DataLoader from '../components/ui/DataLoader.vue';
import BaseButton from '../components/ui/BaseButton.vue';
import BaseBadge from '../components/ui/BaseBadge.vue';
import { useResponsive } from '../composables/useResponsive';

const { fontSm, fontBase, fontXl, isMobile } = useResponsive();

const rawInventory = ref<RawInventoryItem[]>([]);
const preparedInventory = ref<PreparedInventoryItem[]>([]);
const posCategories = ref<POSCategory[]>([]);
const viewMode = ref('raw'); 

const isLoadingData = ref(true);

const isInventoryModalOpen = ref(false);
const isPriceModalOpen = ref(false);
const selectedPreparedItem = ref<PreparedInventoryItem | null>(null);

const expandedGroups = ref<Set<string>>(new Set());

async function loadData() {
  isLoadingData.value = true;
  try {
    rawInventory.value = await inventoryService.getRawInventory();
    preparedInventory.value = await inventoryService.getPreparedInventory();
    posCategories.value = await inventoryService.getPosCategories();
  } catch (error) {
    console.error("Error loading inventory:", error);
  } finally {
    setTimeout(() => { isLoadingData.value = false; }, 600);
  }
}

const groupedPreparedItems = computed(() => {
  const items = preparedInventory.value.filter(item => item.category === viewMode.value);
  const groups: any[] = [];
  const groupMap = new Map();

  items.forEach(item => {
    if (item.variant_group) {
      if (groupMap.has(item.variant_group)) {
        const group = groupMap.get(item.variant_group);
        group.variants.push(item);
        group.total_stock += item.current_stock_pieces;
      } else {
        const newGroup = {
          is_group: true,
          group_name: item.variant_group,
          total_stock: item.current_stock_pieces,
          photo_url: item.photo_url,
          variants: [item]
        };
        groupMap.set(item.variant_group, newGroup);
        groups.push(newGroup);
      }
    } else {
      groups.push({
        is_group: false,
        item: item
      });
    }
  });
  return groups;
});

function toggleGroup(groupName: string) {
  if (expandedGroups.value.has(groupName)) {
    expandedGroups.value.delete(groupName);
  } else {
    expandedGroups.value.add(groupName);
  }
}

function openEditPriceModal(item: PreparedInventoryItem) {
  selectedPreparedItem.value = item;
  isPriceModalOpen.value = true;
}

async function handleSavePrice(data: { prepItemId: number; unitPrice: number; photoUrl: string | null; variantGroup: string | null; variantName: string | null }) {
  try {
    await inventoryService.updatePreparedItemPricing(data.prepItemId, data.unitPrice, data.photoUrl, data.variantGroup, data.variantName);
    isPriceModalOpen.value = false;
    await loadData();
  } catch (error) {
    alert("Failed to update item details.");
  }
}

function getStatusBadge(current: number, threshold: number): 'danger' | 'warning' | 'success' {
  if (current <= threshold * 0.5) return 'danger';
  if (current <= threshold) return 'warning';
  return 'success';
}

function getStatusText(current: number, threshold: number): string {
  if (current <= threshold * 0.5) return 'Critically Low';
  if (current <= threshold) return 'Low Stock';
  return 'Adequate';
}

function handleImageError(event: Event) {
  const target = event.target as HTMLImageElement;
  if (target) {
    target.style.display = 'none';
  }
}

onMounted(() => loadData());
</script>

<template>
  <div class="h-full flex flex-col space-y-6">
    <div class="bg-surface-container-low p-3 rounded-xl shadow-sm border border-outline-variant/15">
      
      <div class="sticky top-0 z-40 bg-surface-container/80 backdrop-blur-xl -mt-3 md:-mt-4 -mx-3 md:-mx-4 px-3 md:px-4 pt-3 md:pt-4 pb-4 mb-6 border-b border-outline-variant/20 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 rounded-t-xl">
        <div>
          <h3 :class="['font-black text-on-surface tracking-tight', fontXl]">Inventory Management</h3>
          <p :class="['text-on-surface-variant mt-1 font-bold tracking-widest uppercase text-[10px]', fontSm]">
            {{ viewMode === 'raw' ? 'Monitored in Kilograms (kg)' : 'Monitored in Sticks/Pieces' }}
          </p>
        </div>
        
        <BaseButton variant="primary" @click="isInventoryModalOpen = true" :class="isMobile ? 'w-full' : ''">
          <svg class="w-4 h-4 mr-2 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
          <span class="tracking-widest uppercase">Edit Inventory</span>
        </BaseButton>
      </div>

      <div class="flex overflow-x-auto pb-3 mb-4 gap-2 border-b border-outline-variant/10">
        <button @click="viewMode = 'raw'" :class="[viewMode === 'raw' ? 'bg-primary-container text-on-primary-container shadow-sm border-primary-container' : 'bg-surface-container text-on-surface-variant hover:bg-surface-container-high border-transparent', 'px-6 py-2.5 font-bold uppercase tracking-widest text-xs rounded-lg transition-colors shrink-0 border']">
          Raw Meats
        </button>
        <button v-for="cat in posCategories" :key="cat.category_name" @click="viewMode = cat.category_name" :class="[viewMode === cat.category_name ? 'bg-primary-container text-on-primary-container shadow-sm border-primary-container' : 'bg-surface-container text-on-surface-variant hover:bg-surface-container-high border-transparent', 'px-6 py-2.5 font-bold uppercase tracking-widest text-xs rounded-lg transition-colors shrink-0 border']">
          {{ cat.category_name }}
        </button>
      </div>

      <DataLoader v-if="isLoadingData" message="Fetching latest inventory records..." />

      <div v-else>
        <div v-if="viewMode === 'raw'" class="overflow-x-auto bg-surface-container-low">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr :class="['border-b border-outline-variant/20 text-on-surface-variant uppercase tracking-widest', fontSm]">
                <th class="pb-3 pt-2 font-bold px-4">Category</th>
                <th class="pb-3 pt-2 font-bold px-4">Specific Part</th>
                <th class="pb-3 pt-2 font-bold px-4">Current Stock</th>
                <th class="pb-3 pt-2 font-bold px-4 hidden md:table-cell">Threshold</th>
                <th class="pb-3 pt-2 font-bold px-4 text-right">Status</th>
              </tr>
            </thead>
            <tbody class="text-on-surface">
              <tr v-if="rawInventory.length === 0">
                <td colspan="5" class="py-8 text-center text-on-surface-variant">No raw inventory found.</td>
              </tr>
              <tr v-for="item in rawInventory" :key="item.raw_item_id" class="border-b border-outline-variant/10 hover:bg-surface-container-high transition-colors">
                <td :class="['py-4 font-black text-on-surface px-4', fontBase]">{{ item.category }}</td>
                <td :class="['py-4 font-bold text-on-surface-variant px-4', fontBase]">{{ item.specific_part }}</td>
                <td :class="['py-4 font-black px-4', item.current_stock_kg <= item.alert_threshold_kg ? 'text-error' : 'text-primary', fontBase]">
                  {{ item.current_stock_kg.toFixed(2) }} kg
                </td>
                <td :class="['py-4 text-on-surface-variant font-bold px-4 hidden md:table-cell', fontBase]">{{ item.alert_threshold_kg }} kg</td>
                <td class="py-4 px-4 text-right">
                  <BaseBadge :text="getStatusText(item.current_stock_kg, item.alert_threshold_kg)" :variant="getStatusBadge(item.current_stock_kg, item.alert_threshold_kg)" />
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div v-else class="overflow-x-auto bg-surface-container-low">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr :class="['border-b border-outline-variant/20 text-on-surface-variant uppercase tracking-widest', fontSm]">
                <th class="pb-3 pt-2 font-bold px-4">Item Name</th>
                <th class="pb-3 pt-2 font-bold px-4">Stock Count</th>
                <th class="pb-3 pt-2 font-bold px-4">Price (PHP)</th>
                <th class="pb-3 pt-2 font-bold px-4 text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="text-on-surface">
              <tr v-if="groupedPreparedItems.length === 0">
                <td colspan="4" class="py-8 text-center text-on-surface-variant">No items found in this category.</td>
              </tr>
              
              <template v-for="(group, index) in groupedPreparedItems" :key="index">
                <tr v-if="!group.is_group" class="border-b border-outline-variant/10 hover:bg-surface-container-high transition-colors">
                  <td :class="['py-3 px-4', fontBase]">
                    <div class="flex items-center gap-4">
                      <div v-if="group.item.photo_url" class="w-12 h-12 rounded-lg bg-surface-container flex items-center justify-center overflow-hidden shrink-0 border border-outline-variant/20 shadow-sm">
                        <img :src="`http://localhost:3000${group.item.photo_url}`" class="w-full h-full object-cover" @error="handleImageError" />
                      </div>
                      <div v-else class="w-12 h-12 rounded-lg bg-surface-container flex items-center justify-center shrink-0 border border-outline-variant/10 text-on-surface-variant opacity-50">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                      </div>
                      <span class="font-black text-on-surface">{{ group.item.pos_display_name }}</span>
                    </div>
                  </td>
                  <td :class="['py-3 font-black text-on-surface px-4', fontBase]">{{ group.item.current_stock_pieces }}</td>
                  <td :class="['py-3 font-black text-primary px-4', fontBase]">₱{{ group.item.unit_price.toFixed(2) }}</td>
                  <td class="py-3 px-4 text-right">
                    <button @click="openEditPriceModal(group.item)" :class="['text-primary hover:text-primary-container font-bold px-3 py-1 transition-colors uppercase tracking-widest', fontSm]">Edit Details</button>
                  </td>
                </tr>

                <tr v-else class="border-b border-outline-variant/10 bg-surface hover:bg-surface-container-high transition-colors cursor-pointer" @click="toggleGroup(group.group_name)">
                  <td :class="['py-3 px-4', fontBase]">
                    <div class="flex items-center gap-4">
                      <div v-if="group.photo_url" class="w-12 h-12 rounded-lg bg-surface-container flex items-center justify-center overflow-hidden shrink-0 border border-outline-variant/20 shadow-sm opacity-90">
                        <img :src="`http://localhost:3000${group.photo_url}`" class="w-full h-full object-cover" @error="handleImageError" />
                      </div>
                      <div v-else class="w-12 h-12 rounded-lg bg-surface-container flex items-center justify-center shrink-0 border border-outline-variant/10 text-on-surface-variant opacity-50">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                      </div>
                      <div class="flex flex-col">
                        <span class="font-black text-on-surface">{{ group.group_name }}</span>
                        <span class="text-[10px] text-primary font-bold mt-0.5 uppercase tracking-widest">{{ group.variants.length }} Variants</span>
                      </div>
                    </div>
                  </td>
                  <td :class="['py-3 font-black text-on-surface px-4', fontBase]">{{ group.total_stock }}</td>
                  <td :class="['py-3 font-bold text-on-surface-variant px-4', fontBase]">Multiple Prices</td>
                  <td class="py-3 px-4 text-right">
                    <button class="text-on-surface-variant hover:text-primary font-bold px-3 py-1 transition-colors uppercase tracking-widest text-xs flex items-center justify-end w-full gap-1">
                      {{ expandedGroups.has(group.group_name) ? 'Hide Variants' : 'Show Variants' }}
                      <svg :class="['w-4 h-4 transition-transform', expandedGroups.has(group.group_name) ? 'rotate-180' : '']" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                    </button>
                  </td>
                </tr>

                <template v-if="group.is_group && expandedGroups.has(group.group_name)">
                  <tr v-for="variant in group.variants" :key="variant.prep_item_id" class="border-b border-outline-variant/5 bg-surface-container-low hover:bg-surface-container transition-colors">
                    <td :class="['py-2 px-4 pl-20', fontBase]">
                      <div class="flex items-center gap-3">
                        <svg class="w-4 h-4 text-outline-variant" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>
                        <span class="font-bold text-on-surface-variant">{{ variant.variant_name || variant.pos_display_name }}</span>
                      </div>
                    </td>
                    <td :class="['py-2 font-bold text-on-surface-variant px-4', fontBase]">{{ variant.current_stock_pieces }}</td>
                    <td :class="['py-2 font-black text-primary px-4', fontBase]">₱{{ variant.unit_price.toFixed(2) }}</td>
                    <td class="py-2 px-4 text-right">
                      <button @click="openEditPriceModal(variant)" :class="['text-primary hover:text-primary-container font-bold px-3 py-1 transition-colors uppercase tracking-widest text-[10px]']">Edit</button>
                    </td>
                  </tr>
                </template>
              </template>
              
            </tbody>
          </table>
        </div>
      </div> 
    </div>

    <EditInventoryModal :is-open="isInventoryModalOpen" @close="isInventoryModalOpen = false" @refresh="loadData" />
    <EditPriceModal :is-open="isPriceModalOpen" :item="selectedPreparedItem" @close="isPriceModalOpen = false; selectedPreparedItem = null" @save="handleSavePrice" />
  </div>
</template>