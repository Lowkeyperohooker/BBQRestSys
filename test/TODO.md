# BBQ Management System - Micro-Step Roadmap

### Phase 1: Core Folders & Services (Completed)
**Goal:** Create the basic folders and the first database service.
- [x] 1.1 Create `components`, `services`, `stores`, and `utils` folders inside `src/`.
- [x] 1.2 Create `src/services/staffService.ts`.
- [x] 1.3 Write the SQLite connection and queries in `staffService.ts`.

### Phase 2: Frontend Structure & Layout (Completed)
**Goal:** Extract layout elements into standalone files and set up navigation.
- [x] 2.1 Inside `src/components`, create a folder named `layout`.
- [x] 2.2 Inside `src/components`, create a folder named `ui`.
- [x] 2.3 Create `src/components/layout/Sidebar.vue` (for the dark navigation menu).
- [x] 2.4 Create `src/components/layout/Header.vue` (for the top bar).
- [x] 2.5 Create `src/components/ui/StaffModal.vue` (for the popup form).
- [x] 2.6 Install Vue Router (`npm install vue-router@4`).
- [x] 2.7 Create `src/router/index.ts` to handle page navigation.
- [x] 2.8 Update `src/main.ts` to activate the router.
- [x] 2.9 Rewrite `App.vue` so it only acts as a container for `<Sidebar>`, `<Header>`, and `<router-view>`.

### Phase 3: Wiring Up the Staff Module (Completed)
**Goal:** Connect the clean UI to the database service.
- [x] 3.1 Update `src/views/Staff.vue` to use the new `staffService`.
- [x] 3.2 Test the app: Click "Add Staff", fill out the modal, and verify it saves to SQLite.

### Phase 4: Meat Inventory & Prep Station
**Goal:** Track raw meat and convert it into ready-to-sell skewers.
- [x] 4.1 Create `src/services/inventoryService.ts`.
- [x] 4.2 Create `src/views/Inventory.vue` with a toggle for Raw vs. Prepared tables.
- [x] 4.3 Create `src/views/PrepStation.vue` with the form for skewering meat.
- [x] 4.4 Update `src/router/index.ts` to add the `/inventory` and `/prep` routes.
- [x] 4.5 Write `logPrepTransaction()` in the service to deduct raw kilos and add skewers.
- [x] 4.6 Test the Prep Station form to ensure inventory updates correctly.

### Phase 5: Point of Sale (POS) & Checkout
**Goal:** Allow cashiers to ring up customers and deduct sold items from stock.
- [x] 5.1 Create `src/services/posService.ts`.
- [x] 5.2 Update `src/views/Pos.vue` to fetch the `Prepared_Inventory` as clickable buttons.
- [x] 5.3 Implement the `cart` logic (add items, calculate Subtotal, Tax, Total).
- [x] 5.4 Write `processCheckout()` in the service to handle the complex SQL transaction.
- [x] 5.5 Test the checkout process and verify inventory goes down.

### Phase 6: Scheduling, System Logs & Dashboard
**Goal:** Track employee hours, audit system actions, and view business analytics.
- [ ] 6.1 Install and configure Pinia (`npm install pinia`) for global state.
- [ ] 6.2 Create `src/stores/authStore.ts` to remember who is logged in.
- [ ] 6.3 Create `src/views/Scheduling.vue` for Clock In / Clock Out functionality.
- [ ] 6.4 Create `src/views/Logs.vue` to display the history of all transactions and prep logs.
- [ ] 6.5 Update `src/router/index.ts` to add the `/schedule` and `/logs` routes.
- [ ] 6.6 Create `src/services/dashboardService.ts` for analytics queries.
- [ ] 6.7 Update `src/views/Dashboard.vue` with Chart.js to show sales trends.
- [ ] 6.8 Run the final build (`npm run tauri build`) to create the `.exe`.