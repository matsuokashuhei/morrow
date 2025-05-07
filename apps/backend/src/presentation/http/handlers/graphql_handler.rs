use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use crate::presentation::graphql::schema::AppSchema;

pub async fn graphql_handler(
    schema: State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
