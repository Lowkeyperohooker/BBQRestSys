# BBQ Management System - Micro-Step Roadmap

### Phase 1: Core Folders & Services (Completed)
**Goal:** Create the basic folders and the first database service.
- [x] 1.1 Create `components`, `services`, `stores`, and `utils` folders inside `src/`.
- [x] 1.2 Create `src/services/staffService.ts`.
- [x] 1.3 Write the SQLite connection and queries in `staffService.ts`.

### Phase 2: Frontend Structure & Layout (WE ARE HERE)
**Goal:** Extract layout elements into standalone files and set up navigation.
- [ ] 2.1 Inside `src/components`, create a folder named `layout`.
- [ ] 2.2 Inside `src/components`, create a folder named `ui`.
- [ ] 2.3 Create `src/components/layout/Sidebar.vue` (for the dark navigation menu).
- [ ] 2.4 Create `src/components/layout/Header.vue` (for the top bar).
- [ ] 2.5 Create `src/components/ui/StaffModal.vue` (for the popup form).
- [ ] 2.6 Install Vue Router (`npm install vue-router@4`).
- [ ] 2.7 Create `src/router/index.ts` to handle page navigation.
- [ ] 2.8 Update `src/main.ts` to activate the router.
- [ ] 2.9 Rewrite `App.vue` so it only acts as a container for `<Sidebar>`, `<Header>`, and `<router-view>`.

### Phase 3: Wiring Up the Staff Module
**Goal:** Connect the clean UI to the database service.
- [ ] 3.1 Update `src/views/Staff.vue` to use the new `staffService`.
- [ ] 3.2 Test the app: Click "Add Staff", fill out the modal, and verify it saves to SQLite.

### Phase 4: Inventory & Kitchen Prep
**Goal:** Track raw meat and convert it into ready-to-sell skewers.
- [ ] 4.1 Create `src/services/inventoryService.ts`.
- [ ] 4.2 Create `src/views/Inventory.vue` with a toggle for Raw vs. Prepared tables.
- [ ] 4.3 Create `src/views/PrepStation.vue` with the form for skewering meat.
- [ ] 4.4 Write `logPrepTransaction()` in the service to deduct raw kilos and add skewers.
- [ ] 4.5 Test the Prep Station form to ensure inventory updates correctly.

### Phase 5: Point of Sale (POS) & Checkout
**Goal:** Allow cashiers to ring up customers and deduct sold items from stock.
- [ ] 5.1 Create `src/services/posService.ts`.
- [ ] 5.2 Update `src/views/Pos.vue` to fetch the `Prepared_Inventory` as clickable buttons.
- [ ] 5.3 Implement the `cart` logic (add items, calculate Subtotal, Tax, Total).
- [ ] 5.4 Write `processCheckout()` in the service to handle the complex SQL transaction.
- [ ] 5.5 Test the checkout process and verify inventory goes down.

### Phase 6: Scheduling & Dashboard
**Goal:** Track employee hours and view business analytics.
- [ ] 6.1 Install and configure Pinia (`npm install pinia`) for global state.
- [ ] 6.2 Create `src/stores/authStore.ts` to remember who is logged in.
- [ ] 6.3 Create `src/views/Scheduling.vue` for Clock In / Clock Out functionality.
- [ ] 6.4 Create `src/services/dashboardService.ts` for analytics queries.
- [ ] 6.5 Update `src/views/Dashboard.vue` with Chart.js to show sales trends.
- [ ] 6.6 Run the final build (`npm run tauri build`) to create the `.exe`.