use axum::{extract::{State, Query}, Json, http::StatusCode};
use sqlx::PgPool;
use serde::Deserialize;
use crate::models::*;

// Structs for extracting Query parameters and JSON bodies
#[derive(Deserialize)]
pub struct StaffQuery { 
    pub staff_id: i32 
}

#[derive(Deserialize)]
pub struct ClockOutReq { 
    pub shift_id: i32, 
    pub staff_id: i32 
}

pub async fn get_today_shifts(State(pool): State<PgPool>) -> AppResult<Vec<Shift>> {
    let shifts = sqlx::query_as::<_, Shift>(
        "SELECT sh.shift_id, sh.staff_id, s.full_name, s.role,
                sh.shift_date, sh.clock_in_time, sh.clock_out_time,
                sh.total_rendered_hours::float8, sh.status
         FROM Shift sh
         JOIN Staff s ON sh.staff_id = s.staff_id
         WHERE sh.shift_date = CURRENT_DATE AT TIME ZONE 'Asia/Manila'
         ORDER BY sh.clock_in_time DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(shifts))
}

pub async fn get_active_shift_for_staff(State(pool): State<PgPool>, Query(q): Query<StaffQuery>) -> AppResult<Option<Shift>> {
    let shift = sqlx::query_as::<_, Shift>(
        "SELECT sh.shift_id, sh.staff_id, s.full_name, s.role,
                sh.shift_date, sh.clock_in_time, sh.clock_out_time,
                sh.total_rendered_hours::float8, sh.status
         FROM Shift sh
         JOIN Staff s ON sh.staff_id = s.staff_id
         WHERE sh.staff_id = $1 AND sh.status = 'Active Shift'"
    )
    .bind(q.staff_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(shift))
}

pub async fn clock_in(State(pool): State<PgPool>, Json(payload): Json<StaffQuery>) -> AppResult<()> {
    let active = sqlx::query_as::<_, (i32,)>(
        "SELECT shift_id FROM Shift WHERE staff_id = $1 AND status = 'Active Shift'"
    )
    .bind(payload.staff_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if active.is_some() {
        return Err((StatusCode::BAD_REQUEST, "Staff member is already clocked in!".to_string()));
    }

    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query(
        "INSERT INTO Shift (staff_id, shift_date, clock_in_time, status)
         VALUES ($1, CURRENT_DATE AT TIME ZONE 'Asia/Manila', NOW(), 'Active Shift')"
    )
    .bind(payload.staff_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description) VALUES ('TIME', $1, 'Clocked In')"
    )
    .bind(payload.staff_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}

pub async fn clock_out(State(pool): State<PgPool>, Json(payload): Json<ClockOutReq>) -> AppResult<()> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query(
        "UPDATE Shift SET
            clock_out_time = NOW(),
            status = 'Completed',
            total_rendered_hours = ROUND(EXTRACT(EPOCH FROM (NOW() - clock_in_time)) / 3600.0, 2)
         WHERE shift_id = $1"
    )
    .bind(payload.shift_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query(
        "INSERT INTO System_Log (log_category, staff_id, description) VALUES ('TIME', $1, 'Clocked Out')"
    )
    .bind(payload.staff_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(()))
}