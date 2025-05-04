use sea_orm::{Database, DatabaseConnection, DbErr};

// データベース接続を確立する関数
pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(database_url).await?;
    Ok(db)
}
