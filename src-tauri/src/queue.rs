use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// Structs for receiving data FROM the frontend Kiosk
#[derive(Deserialize)]
pub struct PendingCartItemReq {
    pub prep_item_id: i32,
    pub qty: i32,
    pub unit_price: f64,
}

#[derive(Deserialize)]
pub struct PendingOrderReq {
    pub queue_number: i32,
    pub order_type: String,
    pub cart_items: Vec<PendingCartItemReq>,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
}

// Structs for sending data TO the frontend Cashier
#[derive(Serialize)]
pub struct PendingOrderRes {
    pub queue_number: i32,
    pub order_type: String,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
    pub timestamp: String,
    pub cart_items: Vec<PendingCartItemRes>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct PendingCartItemRes {
    pub prep_item_id: i32,
    pub pos_display_name: String,
    pub qty: i32,
    pub unit_price: f64,
    pub photo_url: Option<String>,
}

#[derive(sqlx::FromRow)]
struct QueueDbRow {
    queue_id: i32,
    queue_number: i32,
    order_type: String,
    subtotal: f64,
    tax: f64,
    total: f64,
    created_at: chrono::DateTime<chrono::Utc>,
}

// Automatically pulls the next number from 1000 to 9090
pub async fn get_next_number(State(pool): State<PgPool>) -> Result<Json<i32>, (StatusCode, String)> {
    let row: (i64,) = sqlx::query_as("SELECT nextval('kiosk_queue_seq')")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(row.0 as i32))
}

pub async fn add_to_queue(State(pool): State<PgPool>, Json(payload): Json<PendingOrderReq>) -> Result<Json<()>, (StatusCode, String)> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 1. Insert the main order
    let row: (i32,) = sqlx::query_as(
        "INSERT INTO kiosk_queue (queue_number, order_type, subtotal, tax, total) 
         VALUES ($1, $2, $3::numeric, $4::numeric, $5::numeric) RETURNING queue_id"
    )
    .bind(payload.queue_number)
    .bind(&payload.order_type)
    .bind(payload.subtotal)
    .bind(payload.tax)
    .bind(payload.total)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let queue_id = row.0;

    // 2. Insert the specific cart items
    for item in &payload.cart_items {
        sqlx::query(
            "INSERT INTO kiosk_queue_item (queue_id, prep_item_id, quantity, unit_price) 
             VALUES ($1, $2, $3, $4::numeric)"
        )
        .bind(queue_id)
        .bind(item.prep_item_id)
        .bind(item.qty)
        .bind(item.unit_price)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn get_queue(State(pool): State<PgPool>) -> Result<Json<Vec<PendingOrderRes>>, (StatusCode, String)> {
    let queues = sqlx::query_as::<_, QueueDbRow>(
        "SELECT queue_id, queue_number, order_type, subtotal::float8, tax::float8, total::float8, created_at 
         FROM kiosk_queue ORDER BY created_at ASC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut full_queue = Vec::new();

    for q in queues {
        let items = sqlx::query_as::<_, PendingCartItemRes>(
            "SELECT pi.prep_item_id, pi.pos_display_name, kqi.quantity as qty, kqi.unit_price::float8, pi.photo_url 
             FROM kiosk_queue_item kqi 
             JOIN prepared_inventory pi ON kqi.prep_item_id = pi.prep_item_id 
             WHERE kqi.queue_id = $1"
        )
        .bind(q.queue_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        full_queue.push(PendingOrderRes {
            queue_number: q.queue_number,
            order_type: q.order_type,
            subtotal: q.subtotal,
            tax: q.tax,
            total: q.total,
            timestamp: q.created_at.to_string(),
            cart_items: items,
        });
    }

    Ok(Json(full_queue))
}

pub async fn remove_from_queue(State(pool): State<PgPool>, Path(queue_number): Path<i32>) -> Result<Json<()>, (StatusCode, String)> {
    // Because of ON DELETE CASCADE, this will automatically delete the cart items too
    sqlx::query("DELETE FROM kiosk_queue WHERE queue_number = $1")
        .bind(queue_number)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(()))
}