use axum::http::StatusCode;
use axum::response::IntoResponse;

// ヘルスチェックハンドラー
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
