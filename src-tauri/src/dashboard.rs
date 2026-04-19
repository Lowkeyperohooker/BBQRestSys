use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;
use crate::models::*;

pub async fn get_today_sales(State(pool): State<PgPool>) -> AppResult<f64> {
    let row: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(total_amount::float8) FROM Orders WHERE DATE(timestamp AT TIME ZONE 'Asia/Manila') = CURRENT_DATE AT TIME ZONE 'Asia/Manila'"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(row.0.unwrap_or(0.0)))
}

pub async fn get_active_staff_count(State(pool): State<PgPool>) -> AppResult<i64> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM Staff WHERE status = 'Active'"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(row.0))
}

pub async fn get_low_stock_alerts(State(pool): State<PgPool>) -> AppResult<Vec<LowStockAlert>> {
    let alerts = sqlx::query_as::<_, LowStockAlert>(
        "SELECT category, specific_part, current_stock_kg::float8, alert_threshold_kg::float8
         FROM Raw_Inventory WHERE current_stock_kg <= alert_threshold_kg"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(alerts))
}

pub async fn get_top_selling_items(State(pool): State<PgPool>) -> AppResult<Vec<TopSellingItem>> {
    let items = sqlx::query_as::<_, TopSellingItem>(
        "SELECT pi.pos_display_name, SUM(oi.quantity) as total_sold
         FROM Order_Item oi
         JOIN Prepared_Inventory pi ON oi.prep_item_id = pi.prep_item_id
         GROUP BY pi.pos_display_name
         ORDER BY total_sold DESC
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(items))
}