use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    service: String,
    version: String,
    timestamp: String,
}

// Enhanced health check handler
pub async fn health_check() -> impl IntoResponse {
    let health_response = HealthResponse {
        status: "healthy".to_string(),
        service: "morrow-backend".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    (StatusCode::OK, Json(health_response))
}
