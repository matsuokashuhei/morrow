use std::sync::Arc;
use axum::{Router, routing::get, middleware::from_fn_with_state};
use crate::application::services::Services;
use crate::presentation::graphql::schema::AppSchema;
use crate::presentation::http::handlers::graphql_handler::{graphql_handler, graphql_playground};
use crate::presentation::http::middlewares::auth_middleware::auth_middleware;

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
        .layer(from_fn_with_state(services.clone(), auth_middleware))
        .with_state(schema)
        .with_state(services)
        // 他のルートやミドルウェアをここに追加
}

