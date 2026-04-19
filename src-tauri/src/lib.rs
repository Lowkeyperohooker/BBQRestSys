use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tauri::Manager;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

// ============================================================
// STATE
// ============================================================

struct DbState(PgPool);

// ============================================================
// SHARED STRUCTS
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct Staff {
    pub staff_id: i32,
    pub full_name: String,
    pub role: String,
    pub phone_number: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawInventory {
    pub raw_item_id: i32,
    pub category: String,
    pub specific_part: String,
    pub current_stock_kg: Decimal,
    pub alert_threshold_kg: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreparedInventory {
    pub prep_item_id: i32,
    pub raw_item_id: i32,
    pub pos_display_name: String,
    pub current_stock_pieces: i32,
    pub unit_price: Decimal,
    pub is_variable_price: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub order_id: i32,
    pub staff_id: i32,
    pub customer_identifier: String,
    pub order_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderItem {
    pub order_item_id: i32,
    pub order_id: i32,
    pub prep_item_id: i32,
    pub quantity: i32,
    pub price_at_time_of_sale: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrepLog {
    pub prep_log_id: i32,
    pub staff_id: i32,
    pub raw_item_id: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub kilos_deducted: Decimal,
    pub skewers_added: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemLog {
    pub log_id: uuid::Uuid,
    pub log_category: String,
    pub staff_id: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub details: Option<String>,
}

// ============================================================
// STAFF COMMANDS
// ============================================================

#[tauri::command]
async fn get_all_staff(state: tauri::State<'_, DbState>) -> Result<Vec<Staff>, String> {
    let rows = sqlx::query_as!(
        Staff,
        "SELECT staff_id, full_name, role, phone_number, status FROM Staff ORDER BY staff_id"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn add_staff(
    state: tauri::State<'_, DbState>,
    full_name: String,
    role: String,
    phone_number: Option<String>,
) -> Result<Staff, String> {
    let row = sqlx::query_as!(
        Staff,
        "INSERT INTO Staff (full_name, role, phone_number)
         VALUES ($1, $2, $3)
         RETURNING staff_id, full_name, role, phone_number, status",
        full_name,
        role,
        phone_number
    )
    .fetch_one(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row)
}

#[tauri::command]
async fn update_staff_status(
    state: tauri::State<'_, DbState>,
    staff_id: i32,
    status: String,
) -> Result<(), String> {
    sqlx::query!(
        "UPDATE Staff SET status = $1 WHERE staff_id = $2",
        status,
        staff_id
    )
    .execute(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================
// INVENTORY COMMANDS
// ============================================================

#[tauri::command]
async fn get_raw_inventory(state: tauri::State<'_, DbState>) -> Result<Vec<RawInventory>, String> {
    let rows = sqlx::query_as!(
        RawInventory,
        "SELECT raw_item_id, category, specific_part, current_stock_kg, alert_threshold_kg
         FROM Raw_Inventory ORDER BY category, specific_part"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn get_prepared_inventory(
    state: tauri::State<'_, DbState>,
) -> Result<Vec<PreparedInventory>, String> {
    let rows = sqlx::query_as!(
        PreparedInventory,
        "SELECT prep_item_id, raw_item_id, pos_display_name,
                current_stock_pieces, unit_price, is_variable_price
         FROM Prepared_Inventory ORDER BY pos_display_name"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn update_raw_stock(
    state: tauri::State<'_, DbState>,
    raw_item_id: i32,
    new_stock_kg: Decimal,
) -> Result<(), String> {
    sqlx::query!(
        "UPDATE Raw_Inventory SET current_stock_kg = $1 WHERE raw_item_id = $2",
        new_stock_kg,
        raw_item_id
    )
    .execute(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn update_prepared_stock(
    state: tauri::State<'_, DbState>,
    prep_item_id: i32,
    new_stock_pieces: i32,
) -> Result<(), String> {
    sqlx::query!(
        "UPDATE Prepared_Inventory SET current_stock_pieces = $1 WHERE prep_item_id = $2",
        new_stock_pieces,
        prep_item_id
    )
    .execute(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================
// ORDER COMMANDS
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct NewOrderItem {
    pub prep_item_id: i32,
    pub quantity: i32,
    pub price_at_time_of_sale: Decimal,
}

#[tauri::command]
async fn create_order(
    state: tauri::State<'_, DbState>,
    staff_id: i32,
    customer_identifier: String,
    order_type: String,
    subtotal: Decimal,
    tax_amount: Decimal,
    total_amount: Decimal,
    items: Vec<NewOrderItem>,
) -> Result<i32, String> {
    // Use a transaction so order + items are atomic
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    let order = sqlx::query!(
        "INSERT INTO Orders (staff_id, customer_identifier, order_type, subtotal, tax_amount, total_amount)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING order_id",
        staff_id,
        customer_identifier,
        order_type,
        subtotal,
        tax_amount,
        total_amount
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let order_id = order.order_id;

    for item in &items {
        sqlx::query!(
            "INSERT INTO Order_Item (order_id, prep_item_id, quantity, price_at_time_of_sale)
             VALUES ($1, $2, $3, $4)",
            order_id,
            item.prep_item_id,
            item.quantity,
            item.price_at_time_of_sale
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Deduct from prepared inventory stock
        sqlx::query!(
            "UPDATE Prepared_Inventory
             SET current_stock_pieces = current_stock_pieces - $1
             WHERE prep_item_id = $2",
            item.quantity,
            item.prep_item_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(order_id)
}

#[tauri::command]
async fn get_orders(state: tauri::State<'_, DbState>) -> Result<Vec<Order>, String> {
    let rows = sqlx::query_as!(
        Order,
        "SELECT order_id, staff_id, customer_identifier, order_type,
                timestamp, subtotal, tax_amount, total_amount, status
         FROM Orders ORDER BY timestamp DESC"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn update_order_status(
    state: tauri::State<'_, DbState>,
    order_id: i32,
    status: String,
) -> Result<(), String> {
    sqlx::query!(
        "UPDATE Orders SET status = $1 WHERE order_id = $2",
        status,
        order_id
    )
    .execute(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_order_items(
    state: tauri::State<'_, DbState>,
    order_id: i32,
) -> Result<Vec<OrderItem>, String> {
    let rows = sqlx::query_as!(
        OrderItem,
        "SELECT order_item_id, order_id, prep_item_id, quantity, price_at_time_of_sale
         FROM Order_Item WHERE order_id = $1",
        order_id
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

// ============================================================
// PREP LOG COMMANDS
// ============================================================

#[tauri::command]
async fn add_prep_log(
    state: tauri::State<'_, DbState>,
    staff_id: i32,
    raw_item_id: i32,
    kilos_deducted: Decimal,
    skewers_added: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    sqlx::query!(
        "INSERT INTO Prep_Log (staff_id, raw_item_id, kilos_deducted, skewers_added)
         VALUES ($1, $2, $3, $4)",
        staff_id,
        raw_item_id,
        kilos_deducted,
        skewers_added
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // Deduct from raw inventory
    sqlx::query!(
        "UPDATE Raw_Inventory
         SET current_stock_kg = current_stock_kg - $1
         WHERE raw_item_id = $2",
        kilos_deducted,
        raw_item_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_prep_logs(state: tauri::State<'_, DbState>) -> Result<Vec<PrepLog>, String> {
    let rows = sqlx::query_as!(
        PrepLog,
        "SELECT prep_log_id, staff_id, raw_item_id, timestamp, kilos_deducted, skewers_added
         FROM Prep_Log ORDER BY timestamp DESC"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

// ============================================================
// SYSTEM LOG COMMANDS
// ============================================================

#[tauri::command]
async fn add_system_log(
    state: tauri::State<'_, DbState>,
    log_category: String,
    staff_id: i32,
    description: String,
    details: Option<String>,
) -> Result<(), String> {
    sqlx::query!(
        "INSERT INTO System_Log (log_category, staff_id, description, details)
         VALUES ($1, $2, $3, $4)",
        log_category,
        staff_id,
        description,
        details
    )
    .execute(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_system_logs(state: tauri::State<'_, DbState>) -> Result<Vec<SystemLog>, String> {
    let rows = sqlx::query_as!(
        SystemLog,
        "SELECT log_id, log_category, staff_id, timestamp, description, details
         FROM System_Log ORDER BY timestamp DESC"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

// ============================================================
// MISC COMMANDS
// ============================================================

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_system() -> String {
    "BBQ Restaurant System is online and native backend is responding!".to_string()
}

// ============================================================
// ENTRY POINT
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(async {
                PgPoolOptions::new()
                    .max_connections(5)
                    .connect("postgres://postgres:nigmagalaxy@localhost:5432/bbq_system")
                    .await
                    .expect("Failed to connect to PostgreSQL. Is the server running?")
            });

            tauri::async_runtime::block_on(async {
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("Failed to run database migrations");
            });

            app.manage(DbState(pool));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Misc
            greet,
            check_system,
            // Staff
            get_all_staff,
            add_staff,
            update_staff_status,
            // Inventory
            get_raw_inventory,
            get_prepared_inventory,
            update_raw_stock,
            update_prepared_stock,
            // Orders
            create_order,
            get_orders,
            get_order_items,
            update_order_status,
            // Prep Log
            add_prep_log,
            get_prep_logs,
            // System Log
            add_system_log,
            get_system_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}