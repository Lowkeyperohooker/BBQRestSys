use axum::{extract::State, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

#[derive(Deserialize)]
pub struct LoginReq {
    pub passcode: String,
}

// Added FromRow so sqlx knows how to map the database row to this struct at runtime
#[derive(Serialize, FromRow)]
pub struct LoginRes {
    pub staff_id: i32,
    pub full_name: String,
    pub role: String,
}

pub async fn verify_login(
    State(pool): State<PgPool>, 
    Json(payload): Json<LoginReq>
) -> Result<Json<LoginRes>, (StatusCode, String)> {
    
    // Switched to query_as::<_, LoginRes> (Removed the '!' macro)
    let user_result = sqlx::query_as::<_, LoginRes>(
        r#"
        SELECT staff_id, full_name, role 
        FROM Staff 
        WHERE passcode = $1 AND status = 'Active'
        "#
    )
    .bind(&payload.passcode) // Bind the parameter dynamically
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match user_result {
        Some(user) => {
            // Switched to query() (Removed the '!' macro)
            let _ = sqlx::query(
                "INSERT INTO System_Log (log_category, staff_id, description) VALUES ('ADMIN', $1, 'User Logged In')"
            )
            .bind(user.staff_id) // Bind the parameter dynamically
            .execute(&pool).await;

            Ok(Json(user))
        },
        None => Err((StatusCode::UNAUTHORIZED, "Invalid credentials or inactive account".to_string())),
    }
}