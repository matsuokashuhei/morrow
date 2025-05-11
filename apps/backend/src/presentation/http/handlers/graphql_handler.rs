use axum::{
    extract::{Extension, State},
    response::{Html, IntoResponse},
};
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use crate::presentation::graphql::{AppSchema, context::GraphQLContext};

// GraphQL Playground
pub async fn graphql_playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

// GraphQL Handler
pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    Extension(graphql_context): Extension<GraphQLContext>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    // GraphQLリクエストを実行
    let mut request = req.0;
    request = request.data(graphql_context);
    schema.execute(request).await.into()
}
