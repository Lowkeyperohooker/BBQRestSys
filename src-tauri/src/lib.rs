// ============================================================
// ADDITIONAL COMMANDS FOR FRONTEND SERVICES
// ============================================================

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct PrepLogDetailed {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub staff_name: String,
    pub category: String,
    pub specific_part: String,
    pub kilos_deducted: f64,
    pub skewers_added: i32,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct SystemLogDetailed {
    pub log_id: uuid::Uuid,
    pub log_category: String,
    pub staff_id: i32,
    pub staff_name: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Shift {
    pub shift_id: i32,
    pub staff_id: i32,
    pub full_name: String,
    pub role: String,
    pub shift_date: chrono::NaiveDate,
    pub clock_in_time: chrono::DateTime<chrono::Utc>,
    pub clock_out_time: Option<chrono::DateTime<chrono::Utc>>,
    pub total_rendered_hours: Option<f64>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct TopSellingItem {
    pub pos_display_name: String,
    pub total_sold: i64,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct LowStockAlert {
    pub category: String,
    pub specific_part: String,
    pub current_stock_kg: f64,
    pub alert_threshold_kg: f64,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ActiveOrder {
    pub order_id: i32,
    pub customer_identifier: String,
    pub order_type: String,
    pub total_amount: f64,
    pub status: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct OrderReceiptItem {
    pub pos_display_name: String,
    pub quantity: i32,
    pub price_at_time_of_sale: f64,
}

// --- Dashboard ---

#[tauri::command]
async fn get_today_sales(state: tauri::State<'_, DbState>) -> Result<f64, String> {
    let row: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(total_amount::float8) FROM Orders WHERE DATE(timestamp AT TIME ZONE 'Asia/Manila') = CURRENT_DATE AT TIME ZONE 'Asia/Manila'"
    )
    .fetch_one(&state.0)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0.unwrap_or(0.0))
}

#[tauri::command]
async fn get_active_staff_count(state: tauri::State<'_, DbState>) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM Staff WHERE status = 'Active'"
    )
    .fetch_one(&state.0)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
async fn get_low_stock_alerts(state: tauri::State<'_, DbState>) -> Result<Vec<LowStockAlert>, String> {
    sqlx::query_as::<_, LowStockAlert>(
        "SELECT category, specific_part, current_stock_kg::float8, alert_threshold_kg::float8
         FROM Raw_Inventory WHERE current_stock_kg <= alert_threshold_kg"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_top_selling_items(state: tauri::State<'_, DbState>) -> Result<Vec<TopSellingItem>, String> {
    sqlx::query_as::<_, TopSellingItem>(
        "SELECT pi.pos_display_name, SUM(oi.quantity) as total_sold
         FROM Order_Item oi
         JOIN Prepared_Inventory pi ON oi.prep_item_id = pi.prep_item_id
         GROUP BY pi.pos_display_name
         ORDER BY total_sold DESC
         LIMIT 5"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

// --- Inventory ---

#[tauri::command]
async fn add_raw_stock(
    state: tauri::State<'_, DbState>,
    item_id: i32,
    kilos_to_add: f64,
    staff_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    let item: (String, String) = sqlx::query_as(
        "SELECT category, specific_part FROM Raw_Inventory WHERE raw_item_id = $1"
    )
    .bind(item_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let item_name = format!("{} - {}", item.0, item.1);

    sqlx::query(
        "UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg + $1::numeric WHERE raw_item_id = $2"
    )
    .bind(kilos_to_add)
    .bind(item_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let details = format!("Item Restocked: {}\nAmount Added: {:.2} kg", item_name, kilos_to_add);
    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('INVENTORY', $1, 'Stock Delivery Added', $2)"
    )
    .bind(staff_id)
    .bind(details)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn add_new_raw_item(
    state: tauri::State<'_, DbState>,
    category: String,
    part: String,
    initial_kilos: f64,
    alert_threshold: f64,
    staff_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    let row: (i32,) = sqlx::query_as(
        "INSERT INTO Raw_Inventory (category, specific_part, current_stock_kg, alert_threshold_kg)
         VALUES ($1, $2, $3::numeric, $4::numeric) RETURNING raw_item_id"
    )
    .bind(&category)
    .bind(&part)
    .bind(initial_kilos)
    .bind(alert_threshold)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let details = format!(
        "Created Item #{} ({} - {})\nInitial Stock: {:.2} kg\nAlert Threshold: {:.2} kg",
        row.0, category, part, initial_kilos, alert_threshold
    );

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('INVENTORY', $1, 'New Raw Item Added', $2)"
    )
    .bind(staff_id)
    .bind(details)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn update_prepared_item_pricing(
    state: tauri::State<'_, DbState>,
    prep_item_id: i32,
    new_price: f64,
    is_variable: bool,
    staff_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    let item: (String,) = sqlx::query_as(
        "SELECT pos_display_name FROM Prepared_Inventory WHERE prep_item_id = $1"
    )
    .bind(prep_item_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE Prepared_Inventory SET unit_price = $1::numeric, is_variable_price = $2 WHERE prep_item_id = $3"
    )
    .bind(new_price)
    .bind(is_variable)
    .bind(prep_item_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let details = format!(
        "Item: {}\nNew Base Price: ₱{:.2}\nPricing Type: {}",
        item.0, new_price,
        if is_variable { "Variable (Ask Cashier)" } else { "Fixed Price" }
    );

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('INVENTORY', $1, 'Updated Menu Pricing', $2)"
    )
    .bind(staff_id)
    .bind(details)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_available_categories(state: tauri::State<'_, DbState>) -> Result<Vec<String>, String> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT category FROM Raw_Inventory WHERE current_stock_kg > 0 ORDER BY category"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|r| r.0).collect())
}

#[tauri::command]
async fn get_available_parts(
    state: tauri::State<'_, DbState>,
    category: String,
) -> Result<Vec<RawInventory>, String> {
    sqlx::query_as::<_, RawInventory>(
        "SELECT raw_item_id, category, specific_part, current_stock_kg::float8, alert_threshold_kg::float8
         FROM Raw_Inventory WHERE category = $1 AND current_stock_kg > 0 ORDER BY specific_part"
    )
    .bind(category)
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn log_prep_transaction(
    state: tauri::State<'_, DbState>,
    category: String,
    part: String,
    kilos: f64,
    sticks: i32,
    staff_name: Option<String>,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    let raw_item: (i32, f64) = sqlx::query_as(
        "SELECT raw_item_id, current_stock_kg::float8 FROM Raw_Inventory WHERE category = $1 AND specific_part = $2"
    )
    .bind(&category)
    .bind(&part)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| format!("Raw item not found: {} - {}", category, part))?;

    if raw_item.1 < kilos {
        return Err(format!("Insufficient stock! Available: {}kg, Needed: {}kg", raw_item.1, kilos));
    }

    sqlx::query(
        "UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg - $1::numeric WHERE raw_item_id = $2"
    )
    .bind(kilos)
    .bind(raw_item.0)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces + $1 WHERE raw_item_id = $2"
    )
    .bind(sticks)
    .bind(raw_item.0)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let mut actual_staff_id = 1i32;

    if let Some(ref name) = staff_name {
        let staff: Option<(i32,)> = sqlx::query_as(
            "SELECT staff_id FROM Staff WHERE full_name = $1"
        )
        .bind(name)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some((id,)) = staff {
            actual_staff_id = id;
            sqlx::query(
                "INSERT INTO Prep_Log (staff_id, raw_item_id, kilos_deducted, skewers_added)
                 VALUES ($1, $2, $3::numeric, $4)"
            )
            .bind(actual_staff_id)
            .bind(raw_item.0)
            .bind(kilos)
            .bind(sticks)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    let details = format!(
        "Meat Category: {}\nSpecific Part / Cut: {}\nRaw Meat Consumed: {:.2} kg\nYield Produced: {} pieces/sticks\nStaff Member: {}",
        category, part, kilos, sticks,
        staff_name.as_deref().unwrap_or("System Admin")
    );

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('PREP', $1, 'Skewers Prepared', $2)"
    )
    .bind(actual_staff_id)
    .bind(details)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_recent_prep_logs(
    state: tauri::State<'_, DbState>,
    limit: i64,
) -> Result<Vec<PrepLogDetailed>, String> {
    sqlx::query_as::<_, PrepLogDetailed>(
        "SELECT p.timestamp, s.full_name as staff_name, r.category, r.specific_part,
                p.kilos_deducted::float8, p.skewers_added
         FROM Prep_Log p
         JOIN Staff s ON p.staff_id = s.staff_id
         JOIN Raw_Inventory r ON p.raw_item_id = r.raw_item_id
         ORDER BY p.timestamp DESC
         LIMIT $1"
    )
    .bind(limit)
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

// --- POS ---

#[tauri::command]
async fn get_active_orders(state: tauri::State<'_, DbState>) -> Result<Vec<ActiveOrder>, String> {
    sqlx::query_as::<_, ActiveOrder>(
        "SELECT order_id, customer_identifier, order_type, total_amount::float8, status, timestamp
         FROM Orders
         WHERE status != 'Completed' AND status != 'Pending PIN'
         ORDER BY timestamp ASC"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendToGrillItem {
    pub prep_item_id: i32,
    pub qty: i32,
    pub unit_price: f64,
    pub pos_display_name: String,
}

#[tauri::command]
async fn send_to_grill(
    state: tauri::State<'_, DbState>,
    staff_id: i32,
    customer_identifier: String,
    order_type: String,
    cart_items: Vec<SendToGrillItem>,
    subtotal: f64,
    tax: f64,
    total: f64,
) -> Result<i32, String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    let row: (i32,) = sqlx::query_as(
        "INSERT INTO Orders (staff_id, customer_identifier, order_type, subtotal, tax_amount, total_amount, status)
         VALUES ($1, $2, $3, $4::numeric, $5::numeric, $6::numeric, 'Cooking')
         RETURNING order_id"
    )
    .bind(staff_id)
    .bind(&customer_identifier)
    .bind(&order_type)
    .bind(subtotal)
    .bind(tax)
    .bind(total)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let order_id = row.0;

    let mut itemized = String::new();
    for item in &cart_items {
        sqlx::query(
            "INSERT INTO Order_Item (order_id, prep_item_id, quantity, price_at_time_of_sale)
             VALUES ($1, $2, $3, $4::numeric)"
        )
        .bind(order_id)
        .bind(item.prep_item_id)
        .bind(item.qty)
        .bind(item.unit_price)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces - $1 WHERE prep_item_id = $2"
        )
        .bind(item.qty)
        .bind(item.prep_item_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        itemized.push_str(&format!(
            "- {}x {} (PHP {:.2})\n",
            item.qty, item.pos_display_name, item.unit_price * item.qty as f64
        ));
    }

    let details = format!(
        "Order Type: {}\nCustomer/Table: {}\n\nItems Ordered:\n{}\nTotal Amount: PHP {:.2}",
        order_type, customer_identifier, itemized, total
    );

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('POS', $1, 'Order Sent to Grill', $2)"
    )
    .bind(staff_id)
    .bind(details)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(order_id)
}

#[tauri::command]
async fn settle_payment(
    state: tauri::State<'_, DbState>,
    order_id: i32,
    staff_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    let items = sqlx::query_as::<_, OrderReceiptItem>(
        "SELECT pi.pos_display_name, oi.quantity, oi.price_at_time_of_sale::float8
         FROM Order_Item oi
         JOIN Prepared_Inventory pi ON oi.prep_item_id = pi.prep_item_id
         WHERE oi.order_id = $1"
    )
    .bind(order_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let receipt = items.iter().map(|i| {
        format!("- {}x {} (PHP {:.2})", i.quantity, i.pos_display_name, i.price_at_time_of_sale * i.quantity as f64)
    }).collect::<Vec<_>>().join("\n");

    sqlx::query("UPDATE Orders SET status = 'Completed' WHERE order_id = $1")
        .bind(order_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('POS', $1, 'Payment Settled', $2)"
    )
    .bind(staff_id)
    .bind(format!("Order #{} has been paid in full.\n\nOrder Contents:\n{}", order_id, receipt))
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn update_order_status_with_log(
    state: tauri::State<'_, DbState>,
    order_id: i32,
    status: String,
    staff_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    let items = sqlx::query_as::<_, OrderReceiptItem>(
        "SELECT pi.pos_display_name, oi.quantity, oi.price_at_time_of_sale::float8
         FROM Order_Item oi
         JOIN Prepared_Inventory pi ON oi.prep_item_id = pi.prep_item_id
         WHERE oi.order_id = $1"
    )
    .bind(order_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let receipt = items.iter().map(|i| {
        format!("- {}x {} (PHP {:.2})", i.quantity, i.pos_display_name, i.price_at_time_of_sale * i.quantity as f64)
    }).collect::<Vec<_>>().join("\n");

    sqlx::query("UPDATE Orders SET status = $1 WHERE order_id = $2")
        .bind(&status)
        .bind(order_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('POS', $1, 'Order Status Updated', $2)"
    )
    .bind(staff_id)
    .bind(format!("Order #{} marked as {}\n\nOrder Contents:\n{}", order_id, status, receipt))
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

// --- Logs ---

#[tauri::command]
async fn get_recent_logs(
    state: tauri::State<'_, DbState>,
    limit: i64,
) -> Result<Vec<SystemLogDetailed>, String> {
    sqlx::query_as::<_, SystemLogDetailed>(
        "SELECT sl.log_id, sl.log_category, sl.staff_id, s.full_name as staff_name,
                sl.timestamp, sl.description, sl.details
         FROM System_Log sl
         LEFT JOIN Staff s ON sl.staff_id = s.staff_id
         ORDER BY sl.timestamp DESC
         LIMIT $1"
    )
    .bind(limit)
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

// --- Staff ---

#[derive(Serialize, Deserialize, Debug)]
pub struct StaffInput {
    pub name: String,
    pub role: String,
    pub phone: Option<String>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct StaffFull {
    pub staff_id: i32,
    pub full_name: String,
    pub role: String,
    pub phone_number: Option<String>,
    pub status: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[tauri::command]
async fn get_all_staff_full(state: tauri::State<'_, DbState>) -> Result<Vec<StaffFull>, String> {
    sqlx::query_as::<_, StaffFull>(
        "SELECT staff_id, full_name, role, phone_number, status, created_at
         FROM Staff ORDER BY status ASC, full_name ASC"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_staff(
    state: tauri::State<'_, DbState>,
    staff: StaffInput,
    admin_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO Staff (full_name, role, phone_number, status) VALUES ($1, $2, $3, $4)"
    )
    .bind(&staff.name)
    .bind(&staff.role)
    .bind(&staff.phone)
    .bind(&staff.status)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('ADMIN', $1, 'Created Staff Profile', $2)"
    )
    .bind(admin_id)
    .bind(format!("Added {} as {}", staff.name, staff.role))
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn update_staff(
    state: tauri::State<'_, DbState>,
    id: i32,
    staff: StaffInput,
    admin_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE Staff SET full_name = $1, role = $2, phone_number = $3, status = $4 WHERE staff_id = $5"
    )
    .bind(&staff.name)
    .bind(&staff.role)
    .bind(&staff.phone)
    .bind(&staff.status)
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('ADMIN', $1, 'Updated Staff Profile', $2)"
    )
    .bind(admin_id)
    .bind(format!("Updated details for {}", staff.name))
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn delete_staff(
    state: tauri::State<'_, DbState>,
    id: i32,
    admin_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM Staff WHERE staff_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('ADMIN', $1, 'Deleted Staff Profile', $2)"
    )
    .bind(admin_id)
    .bind(format!("Permanently removed staff ID: {}", id))
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn search_staff(
    state: tauri::State<'_, DbState>,
    query: String,
) -> Result<Vec<StaffFull>, String> {
    sqlx::query_as::<_, StaffFull>(
        "SELECT staff_id, full_name, role, phone_number, status, created_at
         FROM Staff WHERE full_name ILIKE $1 OR role ILIKE $1 ORDER BY full_name ASC"
    )
    .bind(format!("%{}%", query))
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

// --- Schedule ---

#[tauri::command]
async fn get_today_shifts(state: tauri::State<'_, DbState>) -> Result<Vec<Shift>, String> {
    sqlx::query_as::<_, Shift>(
        "SELECT sh.shift_id, sh.staff_id, s.full_name, s.role,
                sh.shift_date, sh.clock_in_time, sh.clock_out_time,
                sh.total_rendered_hours::float8, sh.status
         FROM Shift sh
         JOIN Staff s ON sh.staff_id = s.staff_id
         WHERE sh.shift_date = CURRENT_DATE AT TIME ZONE 'Asia/Manila'
         ORDER BY sh.clock_in_time DESC"
    )
    .fetch_all(&state.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_active_shift_for_staff(
    state: tauri::State<'_, DbState>,
    staff_id: i32,
) -> Result<Option<Shift>, String> {
    sqlx::query_as::<_, Shift>(
        "SELECT sh.shift_id, sh.staff_id, s.full_name, s.role,
                sh.shift_date, sh.clock_in_time, sh.clock_out_time,
                sh.total_rendered_hours::float8, sh.status
         FROM Shift sh
         JOIN Staff s ON sh.staff_id = s.staff_id
         WHERE sh.staff_id = $1 AND sh.status = 'Active Shift'"
    )
    .bind(staff_id)
    .fetch_optional(&state.0)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn clock_in(
    state: tauri::State<'_, DbState>,
    staff_id: i32,
) -> Result<(), String> {
    let active = sqlx::query_as::<_, (i32,)>(
        "SELECT shift_id FROM Shift WHERE staff_id = $1 AND status = 'Active Shift'"
    )
    .bind(staff_id)
    .fetch_optional(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    if active.is_some() {
        return Err("Staff member is already clocked in!".to_string());
    }

    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO Shift (staff_id, shift_date, clock_in_time, status)
         VALUES ($1, CURRENT_DATE AT TIME ZONE 'Asia/Manila', NOW(), 'Active Shift')"
    )
    .bind(staff_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description) VALUES ('TIME', $1, 'Clocked In')"
    )
    .bind(staff_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn clock_out(
    state: tauri::State<'_, DbState>,
    shift_id: i32,
    staff_id: i32,
) -> Result<(), String> {
    let mut tx = state.0.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE Shift SET
            clock_out_time = NOW(),
            status = 'Completed',
            total_rendered_hours = ROUND(EXTRACT(EPOCH FROM (NOW() - clock_in_time)) / 3600.0, 2)
         WHERE shift_id = $1"
    )
    .bind(shift_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description) VALUES ('TIME', $1, 'Clocked Out')"
    )
    .bind(staff_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}