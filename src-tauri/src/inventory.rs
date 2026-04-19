use axum::{extract::{State, Query}, Json, http::StatusCode};
use sqlx::PgPool;
use serde::Deserialize;
use crate::models::*;

#[derive(Deserialize)]
pub struct AddRawStockReq { pub item_id: i32, pub kilos_to_add: f64, pub staff_id: i32 }
#[derive(Deserialize)]
pub struct AddNewRawItemReq { pub category: String, pub part: String, pub initial_kilos: f64, pub alert_threshold: f64, pub staff_id: i32 }
#[derive(Deserialize)]
pub struct UpdatePricingReq { pub prep_item_id: i32, pub new_price: f64, pub is_variable: bool, pub staff_id: i32 }
#[derive(Deserialize)]
pub struct CategoryQuery { pub category: String }
#[derive(Deserialize)]
pub struct LogPrepReq { pub category: String, pub part: String, pub kilos: f64, pub sticks: i32, pub staff_name: Option<String> }
#[derive(Deserialize)]
pub struct LimitQuery { pub limit: i64 }

pub async fn get_raw_inventory(State(pool): State<PgPool>) -> AppResult<Vec<RawInventory>> {
    let items = sqlx::query_as::<_, RawInventory>("SELECT * FROM Raw_Inventory ORDER BY category, specific_part")
        .fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn get_prepared_inventory(State(pool): State<PgPool>) -> AppResult<Vec<PreparedInventoryItem>> {
    let items = sqlx::query_as::<_, PreparedInventoryItem>("SELECT * FROM Prepared_Inventory ORDER BY pos_display_name")
        .fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn add_raw_stock(State(pool): State<PgPool>, Json(payload): Json<AddRawStockReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let item: (String, String) = sqlx::query_as("SELECT category, specific_part FROM Raw_Inventory WHERE raw_item_id = $1")
        .bind(payload.item_id).fetch_one(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let item_name = format!("{} - {}", item.0, item.1);

    sqlx::query("UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg + $1::numeric WHERE raw_item_id = $2")
        .bind(payload.kilos_to_add).bind(payload.item_id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let details = format!("Item Restocked: {}\nAmount Added: {:.2} kg", item_name, payload.kilos_to_add);
    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('INVENTORY', $1, 'Stock Delivery Added', $2)")
        .bind(payload.staff_id).bind(details).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn add_new_raw_item(State(pool): State<PgPool>, Json(payload): Json<AddNewRawItemReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let row: (i32,) = sqlx::query_as("INSERT INTO Raw_Inventory (category, specific_part, current_stock_kg, alert_threshold_kg) VALUES ($1, $2, $3::numeric, $4::numeric) RETURNING raw_item_id")
        .bind(&payload.category).bind(&payload.part).bind(payload.initial_kilos).bind(payload.alert_threshold)
        .fetch_one(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let details = format!("Created Item #{} ({} - {})\nInitial Stock: {:.2} kg\nAlert Threshold: {:.2} kg", row.0, payload.category, payload.part, payload.initial_kilos, payload.alert_threshold);
    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('INVENTORY', $1, 'New Raw Item Added', $2)")
        .bind(payload.staff_id).bind(details).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn update_prepared_item_pricing(State(pool): State<PgPool>, Json(payload): Json<UpdatePricingReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let item: (String,) = sqlx::query_as("SELECT pos_display_name FROM Prepared_Inventory WHERE prep_item_id = $1")
        .bind(payload.prep_item_id).fetch_one(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("UPDATE Prepared_Inventory SET unit_price = $1::numeric, is_variable_price = $2 WHERE prep_item_id = $3")
        .bind(payload.new_price).bind(payload.is_variable).bind(payload.prep_item_id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let details = format!("Item: {}\nNew Base Price: ₱{:.2}\nPricing Type: {}", item.0, payload.new_price, if payload.is_variable { "Variable (Ask Cashier)" } else { "Fixed Price" });
    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('INVENTORY', $1, 'Updated Menu Pricing', $2)")
        .bind(payload.staff_id).bind(details).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn get_available_categories(State(pool): State<PgPool>) -> AppResult<Vec<String>> {
    let rows: Vec<(String,)> = sqlx::query_as("SELECT DISTINCT category FROM Raw_Inventory WHERE current_stock_kg > 0 ORDER BY category")
        .fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(rows.into_iter().map(|r| r.0).collect()))
}

pub async fn get_available_parts(State(pool): State<PgPool>, Query(q): Query<CategoryQuery>) -> AppResult<Vec<RawInventory>> {
    let items = sqlx::query_as::<_, RawInventory>(
        "SELECT raw_item_id, category, specific_part, current_stock_kg::float8, alert_threshold_kg::float8
         FROM Raw_Inventory WHERE category = $1 AND current_stock_kg > 0 ORDER BY specific_part"
    )
    .bind(q.category).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn log_prep_transaction(State(pool): State<PgPool>, Json(payload): Json<LogPrepReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let raw_item: (i32, f64) = sqlx::query_as("SELECT raw_item_id, current_stock_kg::float8 FROM Raw_Inventory WHERE category = $1 AND specific_part = $2")
        .bind(&payload.category).bind(&payload.part).fetch_one(&mut *tx).await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Raw item not found".to_string()))?;

    if raw_item.1 < payload.kilos {
        return Err((StatusCode::BAD_REQUEST, "Insufficient stock!".to_string()));
    }

    sqlx::query("UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg - $1::numeric WHERE raw_item_id = $2")
        .bind(payload.kilos).bind(raw_item.0).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces + $1 WHERE raw_item_id = $2")
        .bind(payload.sticks).bind(raw_item.0).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut actual_staff_id = 1i32;

    if let Some(ref name) = payload.staff_name {
        if let Ok(Some((id,))) = sqlx::query_as::<_, (i32,)>("SELECT staff_id FROM Staff WHERE full_name = $1").bind(name).fetch_optional(&mut *tx).await {
            actual_staff_id = id;
            sqlx::query("INSERT INTO Prep_Log (staff_id, raw_item_id, kilos_deducted, skewers_added) VALUES ($1, $2, $3::numeric, $4)")
                .bind(actual_staff_id).bind(raw_item.0).bind(payload.kilos).bind(payload.sticks).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
    }

    let details = format!("Meat Category: {}\nSpecific Part / Cut: {}\nRaw Meat Consumed: {:.2} kg\nYield Produced: {} pieces/sticks\nStaff Member: {}", payload.category, payload.part, payload.kilos, payload.sticks, payload.staff_name.as_deref().unwrap_or("System Admin"));
    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('PREP', $1, 'Skewers Prepared', $2)")
        .bind(actual_staff_id).bind(details).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn get_recent_prep_logs(State(pool): State<PgPool>, Query(q): Query<LimitQuery>) -> AppResult<Vec<PrepLogDetailed>> {
    let logs = sqlx::query_as::<_, PrepLogDetailed>(
        "SELECT p.timestamp, s.full_name as staff_name, r.category, r.specific_part, p.kilos_deducted::float8, p.skewers_added
         FROM Prep_Log p JOIN Staff s ON p.staff_id = s.staff_id JOIN Raw_Inventory r ON p.raw_item_id = r.raw_item_id
         ORDER BY p.timestamp DESC LIMIT $1"
    )
    .bind(q.limit).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(logs))
}