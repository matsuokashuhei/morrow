use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{Extension, State},
    response::{Html, IntoResponse},
};

use crate::presentation::graphql::{AppSchema, context::UserContext};

// GraphQL Playground
pub async fn graphql_playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

// GraphQL Handler
pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    Extension(user): Extension<UserContext>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    // GraphQLリクエストを実行
    let mut request = req.0;
    request = request.data(user);
    schema.execute(request).await.into()
}
