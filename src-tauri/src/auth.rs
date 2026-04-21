use axum::{extract::State, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginReq {
    pub passcode: String, // Can be a PIN or a Password depending on your schema
}

#[derive(Serialize)]
pub struct LoginRes {
    pub staff_id: i32,
    pub full_name: String,
    pub role: String,
}

// Security Note: In a production environment, 'passcode' should be hashed in the database 
// using the pgcrypto extension you configured in your migrations.
pub async fn verify_login(
    State(pool): State<PgPool>, 
    Json(payload): Json<LoginReq>
) -> Result<Json<LoginRes>, (StatusCode, String)> {
    
    // Query the database for an active staff member matching the passcode
    let user_result = sqlx::query_as!(
        LoginRes,
        r#"
        SELECT staff_id, full_name, role 
        FROM Staff 
        WHERE passcode = $1 AND status = 'Active'
        "#,
        payload.passcode
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match user_result {
        Some(user) => {
            // Optional: Log the login event in System_Log
            let _ = sqlx::query!(
                "INSERT INTO System_Log (log_category, staff_id, description) VALUES ('ADMIN', $1, 'User Logged In')",
                user.staff_id
            ).execute(&pool).await;

            Ok(Json(user))
        },
        None => Err((StatusCode::UNAUTHORIZED, "Invalid credentials or inactive account".to_string())),
    }
}