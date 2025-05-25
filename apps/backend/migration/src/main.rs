use std::env;

use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}?currentSchema={}",
        std::env::var("POSTGRES_USER").unwrap(),
        std::env::var("POSTGRES_PASSWORD").unwrap(),
        std::env::var("POSTGRES_HOST").unwrap(),
        std::env::var("POSTGRES_PORT").unwrap(),
        std::env::var("POSTGRES_DB").unwrap(),
        std::env::var("POSTGRES_DB").unwrap(),
    );
    env::set_var("DATABASE_URL", database_url);
    cli::run_cli(migration::Migrator).await;
}
