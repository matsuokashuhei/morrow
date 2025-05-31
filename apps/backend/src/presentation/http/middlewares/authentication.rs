use crate::application::usecases::authenticate_user::AuthenticateUser;
use crate::presentation::graphql::context::UserContext;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn authenticate_user(
    State(authenticate_user): State<Arc<AuthenticateUser>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token_from_headers(&headers);
    match token {
        Some(token) => {
            let user = authenticate_user.execute(&token).await;
            match user {
                Ok(user) => {
                    request
                        .extensions_mut()
                        .insert(UserContext { user: Some(user) });
                    Ok(next.run(request).await)
                }
                Err(_) => {
                    request.extensions_mut().insert(UserContext::default());
                    Ok(next.run(request).await)
                }
            }
        }
        None => {
            request.extensions_mut().insert(UserContext::default());
            Ok(next.run(request).await)
        }
    }
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_string())
            } else {
                None
            }
        })
}
