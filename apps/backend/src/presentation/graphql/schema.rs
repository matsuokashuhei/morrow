use std::sync::Arc;
use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use crate::application::services::Services;

// クエリルート定義
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn hello(&self) -> &'static str {
        "Hello, world!"
    }
    
    // 他のクエリをここに追加
}

// ミューテーションルート定義
pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn echo(&self, text: String) -> String {
        text
    }
    
    // 他のミューテーションをここに追加
}

// スキーマ型エイリアス
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

// スキーマを作成する関数
pub fn create_schema(services: Arc<Services>) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(services)
        .finish()
}

// スキーマビルダーを作成する関数（テスト用）
pub fn create_schema_builder() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
}
