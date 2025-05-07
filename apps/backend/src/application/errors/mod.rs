use thiserror::Error;

// アプリケーションレベルのエラー定義
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("エンティティが見つかりません: {0}")]
    NotFound(String),
    
    #[error("権限がありません: {0}")]
    Unauthorized(String),
    
    #[error("バリデーションエラー: {0}")]
    ValidationError(String),
    
    #[error("データベースエラー: {0}")]
    DatabaseError(String),
    
    #[error("内部エラー: {0}")]
    InternalError(String),
}

// Result型のエイリアス
pub type ApplicationResult<T> = Result<T, ApplicationError>;
