use std::sync::Arc;
use axum::{Router, routing::get};
use crate::application::services::Services;
use crate::presentation::graphql::schema::AppSchema;
use crate::presentation::http::handlers::graphql_handler::{graphql_handler, graphql_playground};

// ヘルスチェックハンドラー
async fn health_check() -> &'static str {
    "Healthy"
}

// ルーターを作成する関数
pub fn create_routes(
    services: Arc<Services>,
    schema: AppSchema,
) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .with_state(schema)
        // 他のルートやミドルウェアをここに追加
}

