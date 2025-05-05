use std::env;

use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        std::env::var("MYSQL_USER").unwrap(),
        std::env::var("MYSQL_PASSWORD").unwrap(),
        std::env::var("MYSQL_HOST").unwrap(),
        std::env::var("MYSQL_PORT").unwrap(),
        std::env::var("MYSQL_DATABASE").unwrap()
    );
    env::set_var("DATABASE_URL", database_url);
    cli::run_cli(migration::Migrator).await;
}
