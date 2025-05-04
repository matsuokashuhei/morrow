use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    // 他の設定項目をここに追加
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            database_url: format!(
                "mysql://{}:{}@{}:{}/{}",
                std::env::var("MYSQL_USER").unwrap(),
                std::env::var("MYSQL_PASSWORD").unwrap(),
                std::env::var("MYSQL_HOST").unwrap(),
                std::env::var("MYSQL_PORT").unwrap(),
                std::env::var("MYSQL_DATABASE").unwrap()
            ), // 他の設定項目の読み込み
        })
    }
}
