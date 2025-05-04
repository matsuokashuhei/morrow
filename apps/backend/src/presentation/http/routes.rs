use std::sync::Arc;
use axum::{Router, routing::get};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::application::services::Services;
use crate::presentation::graphql::schema::AppSchema;

// ヘルスチェックハンドラー
async fn health_check() -> &'static str {
    "Healthy"
}

// GraphQLハンドラー
async fn graphql_handler(
    schema: axum::extract::State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// ルーターを作成する関数
pub fn create_routes(
    services: Arc<Services>,
    schema: AppSchema,
) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/graphql", get(graphql_handler).post(graphql_handler))
        .with_state(schema)
        // 他のルートやミドルウェアをここに追加
}
