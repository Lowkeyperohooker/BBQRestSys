use axum::{extract::{State, Query}, Json, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use crate::models::*;

#[derive(Deserialize)]
pub struct PeriodQuery {
    pub period: Option<String>, // Make optional so existing calls without it don't break
}

#[derive(Serialize, sqlx::FromRow)]
pub struct OrderPoint {
    pub timestamp: String,
    pub amount: f64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct MeatDist {
    pub category: String,
    pub quantity: i64,
}

#[derive(Serialize)]
pub struct PeriodMetrics {
    pub current_sales: f64,
    pub previous_sales: f64,
    pub skewers_sold: i64,
    pub orders: Vec<OrderPoint>,
    pub meat_distribution: Vec<MeatDist>,
}

pub async fn get_today_sales(State(pool): State<PgPool>) -> AppResult<f64> {
    let row: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(total_amount::float8) FROM orders WHERE status = 'Completed' AND DATE(timestamp AT TIME ZONE 'Asia/Manila') = DATE(CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Manila')"
    )
    .fetch_one(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(row.0.unwrap_or(0.0)))
}

pub async fn get_active_staff_count(State(pool): State<PgPool>) -> AppResult<i64> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM staff WHERE status = 'Active'")
    .fetch_one(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(row.0))
}

pub async fn get_low_stock_alerts(State(pool): State<PgPool>) -> AppResult<Vec<LowStockAlert>> {
    let alerts = sqlx::query_as::<_, LowStockAlert>(
        "SELECT category, specific_part, current_stock_kg::float8, alert_threshold_kg::float8
         FROM raw_inventory WHERE current_stock_kg <= alert_threshold_kg"
    )
    .fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(alerts))
}

// FIX: Updated to accept and filter by the period query
pub async fn get_top_selling_items(State(pool): State<PgPool>, Query(q): Query<PeriodQuery>) -> AppResult<Vec<TopSellingItem>> {
    let period = q.period.unwrap_or_else(|| "daily".to_string());
    
    let trunc_unit = match period.as_str() {
        "daily" => "day",
        "weekly" => "week",
        "monthly" => "month",
        "yearly" => "year",
        _ => "day",
    };

    let query = format!(
        "SELECT pi.pos_display_name, SUM(oi.quantity) as total_sold
         FROM order_item oi
         JOIN prepared_inventory pi ON oi.prep_item_id = pi.prep_item_id
         JOIN orders o ON oi.order_id = o.order_id
         WHERE o.status = 'Completed' AND o.timestamp AT TIME ZONE 'Asia/Manila' >= DATE_TRUNC('{}', CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Manila')
         GROUP BY pi.pos_display_name
         ORDER BY total_sold DESC
         LIMIT 5", trunc_unit
    );

    let items = sqlx::query_as::<_, TopSellingItem>(&query)
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(items))
}

pub async fn get_period_metrics(State(pool): State<PgPool>, Query(q): Query<PeriodQuery>) -> AppResult<PeriodMetrics> {
    let period = q.period.unwrap_or_else(|| "daily".to_string());

    let trunc_unit = match period.as_str() {
        "daily" => "day",
        "weekly" => "week",
        "monthly" => "month",
        "yearly" => "year",
        _ => "day",
    };

    // Current Period Sales
    let current_sales_query = format!(
        "SELECT SUM(total_amount::float8) FROM orders WHERE status = 'Completed' AND timestamp AT TIME ZONE 'Asia/Manila' >= DATE_TRUNC('{}', CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Manila')", trunc_unit
    );
    let current_sales: (Option<f64>,) = sqlx::query_as(&current_sales_query).fetch_one(&pool).await.unwrap_or((Some(0.0),));

    // Previous Period Sales (For trend calculation)
    let previous_sales_query = format!(
        "SELECT SUM(total_amount::float8) FROM orders WHERE status = 'Completed' AND timestamp AT TIME ZONE 'Asia/Manila' >= DATE_TRUNC('{}', CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Manila') - INTERVAL '1 {}' AND timestamp AT TIME ZONE 'Asia/Manila' < DATE_TRUNC('{}', CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Manila')", trunc_unit, trunc_unit, trunc_unit
    );
    let previous_sales: (Option<f64>,) = sqlx::query_as(&previous_sales_query).fetch_one(&pool).await.unwrap_or((Some(0.0),));

    // Total Skewers/Units Sold
    let skewers_query = format!(
        "SELECT SUM(oi.quantity) FROM order_item oi JOIN orders o ON oi.order_id = o.order_id WHERE o.status = 'Completed' AND o.timestamp AT TIME ZONE 'Asia/Manila' >= DATE_TRUNC('{}', CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Manila')", trunc_unit
    );
    let skewers: (Option<i64>,) = sqlx::query_as(&skewers_query).fetch_one(&pool).await.unwrap_or((Some(0),));

    // Order Timeline Points for the Line Chart
    let orders_query = format!(
        "SELECT timestamp::text, total_amount::float8 as amount FROM orders WHERE status = 'Completed' AND timestamp AT TIME ZONE 'Asia/Manila' >= DATE_TRUNC('{}', CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Manila') ORDER BY timestamp ASC", trunc_unit
    );
    let orders = sqlx::query_as::<_, OrderPoint>(&orders_query).fetch_all(&pool).await.unwrap_or_default();

    // Meat Distribution for the Doughnut Chart
    let meat_query = format!(
        "SELECT COALESCE(ri.category, 'Other') as category, SUM(oi.quantity) as quantity 
         FROM order_item oi 
         JOIN orders o ON oi.order_id = o.order_id 
         JOIN prepared_inventory pi ON oi.prep_item_id = pi.prep_item_id 
         LEFT JOIN raw_inventory ri ON pi.raw_item_id = ri.raw_item_id 
         WHERE o.status = 'Completed' AND o.timestamp AT TIME ZONE 'Asia/Manila' >= DATE_TRUNC('{}', CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Manila')
         GROUP BY ri.category", trunc_unit
    );
    let meat_dist = sqlx::query_as::<_, MeatDist>(&meat_query).fetch_all(&pool).await.unwrap_or_default();

    Ok(Json(PeriodMetrics {
        current_sales: current_sales.0.unwrap_or(0.0),
        previous_sales: previous_sales.0.unwrap_or(0.0),
        skewers_sold: skewers.0.unwrap_or(0),
        orders,
        meat_distribution: meat_dist,
    }))
}