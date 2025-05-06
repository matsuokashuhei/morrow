mod application;
mod domain;
mod infrastructure;
mod presentation;

use aws_config::BehaviorVersion;
use dotenvy::dotenv;
use infrastructure::config::app_config::AppConfig;
use presentation::graphql::schema::create_schema;
use presentation::http::routes::create_routes;
use std::sync::Arc;
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
        infrastructure::database::repositories::init_repositories(connection.clone(), &sdk_config);
    info!("Repositories initialized");

    // アプリケーションサービスの初期化
    let services = application::services::init_services(Arc::new(repositories));
    info!("Application services initialized");

    // GraphQLスキーマの作成
    let schema = create_schema(&services);
    info!("GraphQL schema created");

    // HTTPルーターの作成
    let router = create_routes(Arc::new(services), schema);
    info!("HTTP router created");

    // サーバーの起動
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;
    Ok(())
}
