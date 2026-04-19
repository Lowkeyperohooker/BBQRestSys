use axum::{extract::{State, Query}, Json, http::StatusCode};
use sqlx::PgPool;
use crate::models::*;
use crate::inventory::LimitQuery;

pub async fn get_recent_logs(State(pool): State<PgPool>, Query(q): Query<LimitQuery>) -> AppResult<Vec<SystemLogDetailed>> {
    let logs = sqlx::query_as::<_, SystemLogDetailed>(
        "SELECT sl.log_id, sl.log_category, sl.staff_id, s.full_name as staff_name, sl.timestamp, sl.description, sl.details
         FROM System_Log sl LEFT JOIN Staff s ON sl.staff_id = s.staff_id ORDER BY sl.timestamp DESC LIMIT $1"
    )
    .bind(q.limit)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(logs))
}