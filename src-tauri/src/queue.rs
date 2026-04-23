use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tokio::fs;

// The file will be created in your src-tauri folder
const QUEUE_FILE: &str = "pending_kiosk_orders.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct PendingCartItem {
    pub prep_item_id: i32,
    pub raw_item_id: i32,
    pub pos_display_name: String,
    pub current_stock_pieces: i32,
    pub unit_price: f64,
    pub is_variable_price: bool,
    pub qty: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PendingOrder {
    pub queue_number: i32,
    pub order_type: String,
    pub cart_items: Vec<PendingCartItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
    pub timestamp: String,
}

// Helper to safely read the queue file
async fn read_queue_file() -> Vec<PendingOrder> {
    match fs::read_to_string(QUEUE_FILE).await {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| vec![]),
        Err(_) => vec![], // If file doesn't exist yet, return an empty array
    }
}

// Helper to securely save the queue file
async fn write_queue_file(queue: &Vec<PendingOrder>) -> Result<(), String> {
    let json = serde_json::to_string_pretty(queue).map_err(|e| e.to_string())?;
    fs::write(QUEUE_FILE, json).await.map_err(|e| e.to_string())?;
    Ok(())
}

// GET: Fetch all pending orders for the Cashier
pub async fn get_queue() -> Result<Json<Vec<PendingOrder>>, (StatusCode, String)> {
    let queue = read_queue_file().await;
    Ok(Json(queue))
}

// POST: Kiosk sends a new order to the queue
pub async fn add_to_queue(Json(new_order): Json<PendingOrder>) -> Result<Json<()>, (StatusCode, String)> {
    let mut queue = read_queue_file().await;
    
    queue.push(new_order);

    // Enforce 200 item limit (FIFO: removes the oldest items if array exceeds 200)
    if queue.len() > 200 {
        let excess = queue.len() - 200;
        queue.drain(0..excess);
    }

    write_queue_file(&queue)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(()))
}

// POST: Cashier accepts/clears an order from the queue
pub async fn remove_from_queue(Path(queue_number): Path<i32>) -> Result<Json<()>, (StatusCode, String)> {
    let mut queue = read_queue_file().await;
    
    // Keep all orders EXCEPT the one matching the queue number
    queue.retain(|order| order.queue_number != queue_number);

    write_queue_file(&queue)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(()))
}