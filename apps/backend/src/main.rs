mod application;
mod domain;
mod infrastructure;
mod presentation;

use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use dotenvy::dotenv;
use infrastructure::config::app_config::AppConfig;
use presentation::http::handlers::graphql_handler::{graphql_handler, graphql_playground};
use presentation::http::middlewares::auth_middleware::auth_middleware;
use presentation::{graphql::schema::build_schema, http::handlers::health::health_check};
use std::sync::Arc;
use tower::ServiceBuilder;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数のロード
    dotenv().ok();

    // トレースの初期化
    // tracing_subscriber::fmt::init();
    // トレースの初期化 (デバッグレベルで設定)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    // アプリケーション設定の読み込み
    let config = AppConfig::from_env()?;
    info!("Application configuration loaded");

    // データベース接続の確立
    let connection =
        infrastructure::database::connection::establish_connection(&config.database_url).await?;
    info!("Database connection established");

    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    info!("AWS SDK configuration loaded");

    // リポジトリの初期化
    let repositories =
        infrastructure::database::repositories::init_repositories(connection.clone());
    info!("Repositories initialized");

    let authentication_service =
        Arc::new(infrastructure::authentication::cognito_service::CognitoService::new(&sdk_config));

    // アプリケーションサービスの初期化
    let services = application::services::init_services(Arc::new(repositories.clone())).await;
    info!("Application services initialized");

    let use_cases = application::usecases::init_use_cases(
        Arc::new(repositories.clone()),
        authentication_service,
    );

    // GraphQLスキーマの作成
    let schema = build_schema(&use_cases, &services);
    info!("GraphQL schema created");

    // HTTPルーターの作成
    // let router = build_routes(Arc::new(services), schema);
    let router = Router::new()
        .route("/health", get(health_check))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(ServiceBuilder::new().layer(from_fn_with_state(
            use_cases.authenticate_user,
            auth_middleware,
        )))
        .with_state(schema)
        .with_state(services);

    // サーバーの起動
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;
    Ok(())
}
