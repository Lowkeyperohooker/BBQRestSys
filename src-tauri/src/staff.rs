use axum::{extract::{State, Query}, Json, http::StatusCode};
use sqlx::PgPool;
use serde::Deserialize;
use crate::models::*;

// Structs for extracting Query parameters and JSON bodies
#[derive(Deserialize)]
pub struct StaffAdminReq { 
    pub staff: StaffInput, 
    pub admin_id: i32 
}

#[derive(Deserialize)]
pub struct UpdateStaffReq { 
    pub id: i32, 
    pub staff: StaffInput, 
    pub admin_id: i32 
}

#[derive(Deserialize)]
pub struct DeleteStaffReq { 
    pub id: i32, 
    pub admin_id: i32 
}

#[derive(Deserialize)]
pub struct SearchQuery { 
    pub query: String 
}

pub async fn get_all_staff_full(State(pool): State<PgPool>) -> AppResult<Vec<StaffFull>> {
    let staff = sqlx::query_as::<_, StaffFull>("SELECT staff_id, full_name, role, phone_number, status, created_at FROM Staff ORDER BY status ASC, full_name ASC")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    Ok(Json(staff))
}

pub async fn create_staff(State(pool): State<PgPool>, Json(payload): Json<StaffAdminReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    sqlx::query("INSERT INTO Staff (full_name, role, phone_number, status) VALUES ($1, $2, $3, $4)")
        .bind(&payload.staff.name).bind(&payload.staff.role).bind(&payload.staff.phone).bind(&payload.staff.status)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('ADMIN', $1, 'Created Staff Profile', $2)")
        .bind(payload.admin_id).bind(format!("Added {} as {}", payload.staff.name, payload.staff.role))
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn update_staff(State(pool): State<PgPool>, Json(payload): Json<UpdateStaffReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    sqlx::query("UPDATE Staff SET full_name = $1, role = $2, phone_number = $3, status = $4 WHERE staff_id = $5")
        .bind(&payload.staff.name).bind(&payload.staff.role).bind(&payload.staff.phone).bind(&payload.staff.status).bind(payload.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('ADMIN', $1, 'Updated Staff Profile', $2)")
        .bind(payload.admin_id).bind(format!("Updated details for {}", payload.staff.name))
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn delete_staff(State(pool): State<PgPool>, Json(payload): Json<DeleteStaffReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    sqlx::query("DELETE FROM Staff WHERE staff_id = $1")
        .bind(payload.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("INSERT INTO System_Log (log_category, staff_id, description, details) VALUES ('ADMIN', $1, 'Deleted Staff Profile', $2)")
        .bind(payload.admin_id).bind(format!("Permanently removed staff ID: {}", payload.id))
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn search_staff(State(pool): State<PgPool>, Query(q): Query<SearchQuery>) -> AppResult<Vec<StaffFull>> {
    let staff = sqlx::query_as::<_, StaffFull>("SELECT staff_id, full_name, role, phone_number, status, created_at FROM Staff WHERE full_name ILIKE $1 OR role ILIKE $1 ORDER BY full_name ASC")
        .bind(format!("%{}%", q.query))
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    Ok(Json(staff))
}