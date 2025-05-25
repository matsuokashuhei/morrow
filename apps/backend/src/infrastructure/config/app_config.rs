use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub aws_cognito_user_pool_client_id: String,
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
                "postgres://{}:{}@{}:{}/{}",
                std::env::var("POSTGRES_USER").unwrap(),
                std::env::var("POSTGRES_PASSWORD").unwrap(),
                std::env::var("POSTGRES_HOST").unwrap(),
                std::env::var("POSTGRES_PORT").unwrap(),
                std::env::var("POSTGRES_DB").unwrap()
            ),
            aws_cognito_user_pool_client_id: env::var("AWS_COGNITO_USER_POOL_CLIENT_ID").unwrap(),
        })
    }
}
