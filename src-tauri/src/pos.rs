use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;
use serde::Deserialize;
use crate::models::*;

// Structs for extracting JSON payload data
#[derive(Deserialize)]
pub struct SendToGrillReq { 
    pub staff_id: i32, 
    pub customer_identifier: String, 
    pub order_type: String, 
    pub cart_items: Vec<SendToGrillItem>, 
    pub subtotal: f64, 
    pub tax: f64, 
    pub total: f64 
}

#[derive(Deserialize)]
pub struct OrderStaffReq { 
    pub order_id: i32, 
    pub staff_id: i32 
}

#[derive(Deserialize)]
pub struct UpdateStatusReq { 
    pub order_id: i32, 
    pub status: String, 
    pub staff_id: i32 
}

pub async fn get_active_orders(State(pool): State<PgPool>) -> AppResult<Vec<ActiveOrder>> {
    let orders = sqlx::query_as::<_, ActiveOrder>(
        "SELECT order_id, customer_identifier, order_type, total_amount::float8, status, timestamp
         FROM Orders WHERE status != 'Completed' AND status != 'Pending PIN' ORDER BY timestamp ASC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(orders))
}

pub async fn send_to_grill(State(pool): State<PgPool>, Json(payload): Json<SendToGrillReq>) -> AppResult<i32> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let row: (i32,) = sqlx::query_as(
        "INSERT INTO Orders (staff_id, customer_identifier, order_type, subtotal, tax_amount, total_amount, status)
         VALUES ($1, $2, $3, $4::numeric, $5::numeric, $6::numeric, 'Cooking') RETURNING order_id"
    )
    .bind(payload.staff_id).bind(&payload.customer_identifier).bind(&payload.order_type)
    .bind(payload.subtotal).bind(payload.tax).bind(payload.total)
    .fetch_one(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let order_id = row.0;
    let mut itemized = String::new();

    for item in &payload.cart_items {
        sqlx::query("INSERT INTO Order_Item (order_id, prep_item_id, quantity, price_at_time_of_sale) VALUES ($1, $2, $3, $4::numeric)")
            .bind(order_id).bind(item.prep_item_id).bind(item.qty).bind(item.unit_price)
            .execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        sqlx::query("UPDATE Prepared_Inventory SET current_stock_pieces = current_stock_pieces - $1 WHERE prep_item_id = $2")
            .bind(item.qty).bind(item.prep_item_id)
            .execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        itemized.push_str(&format!("- {}x {} (PHP {:.2})\n", item.qty, item.pos_display_name, item.unit_price * item.qty as f64));
    }

    let details = format!("Order Type: {}\nCustomer/Table: {}\n\nItems Ordered:\n{}\nTotal Amount: PHP {:.2}", payload.order_type, payload.customer_identifier, itemized, payload.total);
    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('POS', $1, 'Order Sent to Grill', $2)")
        .bind(payload.staff_id).bind(details).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(order_id))
}

pub async fn settle_payment(State(pool): State<PgPool>, Json(payload): Json<OrderStaffReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let items = sqlx::query_as::<_, OrderReceiptItem>(
        "SELECT pi.pos_display_name, oi.quantity, oi.price_at_time_of_sale::float8 FROM Order_Item oi JOIN Prepared_Inventory pi ON oi.prep_item_id = pi.prep_item_id WHERE oi.order_id = $1"
    ).bind(payload.order_id).fetch_all(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let receipt = items.iter().map(|i| format!("- {}x {} (PHP {:.2})", i.quantity, i.pos_display_name, i.price_at_time_of_sale * i.quantity as f64)).collect::<Vec<_>>().join("\n");

    sqlx::query("UPDATE Orders SET status = 'Completed' WHERE order_id = $1").bind(payload.order_id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('POS', $1, 'Payment Settled', $2)")
        .bind(payload.staff_id).bind(format!("Order #{} has been paid in full.\n\nOrder Contents:\n{}", payload.order_id, receipt))
        .execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn update_order_status_with_log(State(pool): State<PgPool>, Json(payload): Json<UpdateStatusReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let items = sqlx::query_as::<_, OrderReceiptItem>(
        "SELECT pi.pos_display_name, oi.quantity, oi.price_at_time_of_sale::float8 FROM Order_Item oi JOIN Prepared_Inventory pi ON oi.prep_item_id = pi.prep_item_id WHERE oi.order_id = $1"
    ).bind(payload.order_id).fetch_all(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let receipt = items.iter().map(|i| format!("- {}x {} (PHP {:.2})", i.quantity, i.pos_display_name, i.price_at_time_of_sale * i.quantity as f64)).collect::<Vec<_>>().join("\n");

    sqlx::query("UPDATE Orders SET status = $1 WHERE order_id = $2").bind(&payload.status).bind(payload.order_id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('POS', $1, 'Order Status Updated', $2)")
        .bind(payload.staff_id).bind(format!("Order #{} marked as {}\n\nOrder Contents:\n{}", payload.order_id, payload.status, receipt))
        .execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}