use axum::{extract::{State, Query}, Json, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use crate::models::*;

// --- Request Payloads ---
#[derive(Deserialize)]
pub struct EditStockReq { pub item_type: String, pub item_id: i32, pub quantity_change: f64, pub reason: String, pub staff_id: i32 }
#[derive(Deserialize)]
pub struct AddNewRawItemReq { pub category: String, pub part: String, pub initial_kilos: f64, pub alert_threshold: f64, pub staff_id: i32 }
#[derive(Deserialize)]
pub struct AddPreparedReq { pub category: String, pub pos_display_name: String, pub unit_price: f64, pub is_variable: bool, pub staff_id: i32 }
#[derive(Deserialize)]
pub struct UpdatePricingReq { pub prep_item_id: i32, pub new_price: f64, pub is_variable: bool, pub staff_id: i32 }
#[derive(Deserialize)]
pub struct PosCategoryReq { pub category_name: String }
#[derive(Deserialize)]
pub struct CategoryQuery { pub category: String }
#[derive(Deserialize)]
pub struct LogPrepReq { pub category: String, pub part: String, pub kilos: f64, pub sticks: i32, pub staff_name: Option<String> }
#[derive(Deserialize)]
pub struct LimitQuery { pub limit: i64 }

// --- Response Payloads ---
#[derive(Serialize, sqlx::FromRow)]
pub struct PosCategoryResp { pub category_name: String, pub is_removable: bool }

#[derive(Serialize, sqlx::FromRow)]
pub struct PreparedInvResp {
    pub prep_item_id: i32,
    pub raw_item_id: Option<i32>,
    pub category: String,
    pub pos_display_name: String,
    pub current_stock_pieces: i32,
    pub unit_price: f64,
    pub is_variable_price: bool,
}

// --- Endpoints ---
pub async fn get_raw_inventory(State(pool): State<PgPool>) -> AppResult<Vec<RawInventory>> {
    let items = sqlx::query_as::<_, RawInventory>(
        "SELECT raw_item_id, category, specific_part, current_stock_kg::float8, alert_threshold_kg::float8 
         FROM Raw_Inventory ORDER BY category, specific_part"
    ).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn get_prepared_inventory(State(pool): State<PgPool>) -> AppResult<Vec<PreparedInvResp>> {
    let items = sqlx::query_as::<_, PreparedInvResp>(
        "SELECT prep_item_id, raw_item_id, category, pos_display_name, current_stock_pieces, unit_price::float8, is_variable_price 
         FROM Prepared_Inventory ORDER BY pos_display_name"
    ).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn get_pos_categories(State(pool): State<PgPool>) -> AppResult<Vec<PosCategoryResp>> {
    let cats = sqlx::query_as::<_, PosCategoryResp>("SELECT category_name, is_removable FROM POS_Category ORDER BY is_removable ASC, category_name ASC")
        .fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(cats))
}

pub async fn add_pos_category(State(pool): State<PgPool>, Json(payload): Json<PosCategoryReq>) -> AppResult<()> {
    sqlx::query("INSERT INTO POS_Category (category_name, is_removable) VALUES ($1, TRUE)")
        .bind(payload.category_name).execute(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn remove_pos_category(State(pool): State<PgPool>, Json(payload): Json<PosCategoryReq>) -> AppResult<()> {
    // Fails safely if items exist in this category due to foreign keys, or we can just delete the tab
    sqlx::query("DELETE FROM POS_Category WHERE category_name = $1 AND is_removable = TRUE")
        .bind(payload.category_name).execute(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn edit_stock(State(pool): State<PgPool>, Json(payload): Json<EditStockReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let details = format!("Adjustment: {:+.2}\nReason: {}", payload.quantity_change, payload.reason);

    if payload.item_type == "raw" {
        sqlx::query("UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg + $1::numeric WHERE raw_item_id = $2")
            .bind(payload.quantity_change).bind(payload.item_id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('INVENTORY', $1, 'Raw Stock Adjusted', $2)")
            .bind(payload.staff_id).bind(details).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    } else {
        sqlx::query("UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces + $1 WHERE prep_item_id = $2")
            .bind(payload.quantity_change as i32).bind(payload.item_id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
        sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('INVENTORY', $1, 'Prepared Stock Adjusted', $2)")
            .bind(payload.staff_id).bind(details).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn add_new_raw_item(State(pool): State<PgPool>, Json(payload): Json<AddNewRawItemReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    sqlx::query("INSERT INTO Raw_Inventory (category, specific_part, current_stock_kg, alert_threshold_kg) VALUES ($1, $2, $3::numeric, $4::numeric)")
        .bind(&payload.category).bind(&payload.part).bind(payload.initial_kilos).bind(payload.alert_threshold)
        .execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn add_prepared_item(State(pool): State<PgPool>, Json(payload): Json<AddPreparedReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    sqlx::query("INSERT INTO Prepared_Inventory (category, pos_display_name, unit_price, is_variable_price) VALUES ($1, $2, $3::numeric, $4)")
        .bind(payload.category).bind(payload.pos_display_name).bind(payload.unit_price).bind(payload.is_variable)
        .execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn update_prepared_item_pricing(State(pool): State<PgPool>, Json(payload): Json<UpdatePricingReq>) -> AppResult<()> {
    sqlx::query("UPDATE Prepared_Inventory SET unit_price = $1::numeric, is_variable_price = $2 WHERE prep_item_id = $3")
        .bind(payload.new_price).bind(payload.is_variable).bind(payload.prep_item_id)
        .execute(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
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
    ).bind(q.category).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn log_prep_transaction(State(pool): State<PgPool>, Json(payload): Json<LogPrepReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let raw_item: (i32, f64) = sqlx::query_as("SELECT raw_item_id, current_stock_kg::float8 FROM Raw_Inventory WHERE category = $1 AND specific_part = $2")
        .bind(&payload.category).bind(&payload.part).fetch_one(&mut *tx).await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Raw item not found".to_string()))?;

    if raw_item.1 < payload.kilos { return Err((StatusCode::BAD_REQUEST, "Insufficient stock!".to_string())); }

    sqlx::query("UPDATE Raw_Inventory SET current_stock_kg = current_stock_kg - $1::numeric WHERE raw_item_id = $2")
        .bind(payload.kilos).bind(raw_item.0).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces + $1 WHERE raw_item_id = $2")
        .bind(payload.sticks).bind(raw_item.0).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn get_recent_prep_logs(State(pool): State<PgPool>, Query(q): Query<LimitQuery>) -> AppResult<Vec<PrepLogDetailed>> {
    let logs = sqlx::query_as::<_, PrepLogDetailed>(
        "SELECT p.timestamp, s.full_name as staff_name, r.category, r.specific_part, p.kilos_deducted::float8, p.skewers_added
         FROM Prep_Log p JOIN Staff s ON p.staff_id = s.staff_id JOIN Raw_Inventory r ON p.raw_item_id = r.raw_item_id
         ORDER BY p.timestamp DESC LIMIT $1"
    ).bind(q.limit).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(logs))
}