use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct DbState(pub PgPool);

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct RawInventory {
    pub raw_item_id: i32,
    pub category: String,
    pub specific_part: String,
    pub current_stock_kg: f64,
    pub alert_threshold_kg: f64,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct SendToGrillItem {
    pub prep_item_id: i32,
    pub qty: i32,
    pub unit_price: f64,
    pub pos_display_name: String,
}

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