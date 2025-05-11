use std::sync::Arc;
use axum::{
    extract::{Request, State},
    middleware::Next,
    http::{HeaderMap, StatusCode},
    response::Response,
};
use crate::application::services::Services;
use crate::presentation::graphql::context::GraphQLContext;

pub async fn auth_middleware(
    State(services): State<Arc<Services>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // トークンの取得
    let token = extract_token_from_headers(&headers);
    
    // トークンからユーザーコンテキストを生成
    match services.authorization_service.get_auth_context(token.as_deref()).await {
        Ok(auth_context) => {
            // GraphQLコンテキストの作成
            let graphql_context = GraphQLContext::new(auth_context);
            
            // リクエストにGraphQLコンテキストをエクステンションとして追加
            request.extensions_mut().insert(graphql_context);
            
            // 次のハンドラへ処理を移譲
            Ok(next.run(request).await)
        }
        Err(err) => {
            // トークン検証エラー時は、空のコンテキストを追加（ゲストアクセス扱い）
            tracing::warn!("Token validation error: {:?}", err);
            let graphql_context = GraphQLContext::default();
            request.extensions_mut().insert(graphql_context);
            
            // 処理を続行（認証が必須のエンドポイントはリゾルバ内でチェック）
            Ok(next.run(request).await)
        }
    }
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header[7..].to_string())
            } else {
                None
            }
        })
}
