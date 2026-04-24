use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tokio::fs;

// FIX: Added "../" to save these files in the root folder, outside the Tauri watcher
const QUEUE_FILE: &str = "../pending_kiosk_orders.json";
const COUNTER_FILE: &str = "../queue_counter.txt";

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

// --- Persistent Counter Logic ---
async fn read_counter() -> i32 {
    match fs::read_to_string(COUNTER_FILE).await {
        // Default to 999 so the very first order generated is 1000
        Ok(contents) => contents.trim().parse().unwrap_or(999),
        Err(_) => 999, 
    }
}

async fn write_counter(num: i32) -> Result<(), String> {
    fs::write(COUNTER_FILE, num.to_string()).await.map_err(|e| e.to_string())
}

pub async fn get_next_number() -> Result<Json<i32>, (StatusCode, String)> {
    let mut num = read_counter().await;
    
    num += 1;
    
    // Strict enforcement of 1000 to 9090 range
    if num > 9090 {
        num = 1000;
    }
    
    write_counter(num).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    Ok(Json(num))
}

// --- JSON Queue Logic ---
async fn read_queue_file() -> Vec<PendingOrder> {
    match fs::read_to_string(QUEUE_FILE).await {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

async fn write_queue_file(queue: &Vec<PendingOrder>) -> Result<(), String> {
    let json = serde_json::to_string_pretty(queue).map_err(|e| e.to_string())?;
    fs::write(QUEUE_FILE, json).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_queue() -> Result<Json<Vec<PendingOrder>>, (StatusCode, String)> {
    let queue = read_queue_file().await;
    Ok(Json(queue))
}

pub async fn add_to_queue(Json(new_order): Json<PendingOrder>) -> Result<Json<()>, (StatusCode, String)> {
    let mut queue = read_queue_file().await;
    
    queue.push(new_order);

    if queue.len() > 200 {
        let excess = queue.len() - 200;
        queue.drain(0..excess);
    }

    write_queue_file(&queue)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(()))
}

pub async fn remove_from_queue(Path(queue_number): Path<i32>) -> Result<Json<()>, (StatusCode, String)> {
    let mut queue = read_queue_file().await;
    
    queue.retain(|order| order.queue_number != queue_number);

    write_queue_file(&queue)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(()))
}