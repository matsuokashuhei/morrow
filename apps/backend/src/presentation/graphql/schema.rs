use crate::application::services::Services;
use crate::domain::repositories::user_repository::UserRepository;
use crate::presentation::graphql::object::user_object::{UserMutation, UserQuery};
use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use std::sync::Arc;

// クエリルート定義
pub struct QueryRoot {
    user_query: UserQuery,
    // 他のクエリをここに追加
}

#[async_graphql::Object]
impl QueryRoot {
    async fn hello(&self) -> &'static str {
        "Hello, world!"
    }

    // ユーザークエリへのアクセスを提供
    async fn users(&self) -> &UserQuery {
        &self.user_query
    }
}

// ミューテーションルート定義
pub struct MutationRoot {
    user_mutation: UserMutation,
    // 他のミューテーションをここに追加
}

#[async_graphql::Object]
impl MutationRoot {
    async fn echo(&self, text: String) -> String {
        text
    }

    // ユーザーミューテーションへのアクセスを提供
    async fn users(&self) -> &UserMutation {
        &self.user_mutation
    }
}

// スキーマ型エイリアス
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

// スキーマを作成する関数
pub fn create_schema(services: &Services) -> AppSchema {
    let user_query = UserQuery::new(Arc::clone(&services.user_service));
    let user_mutation = UserMutation::new(Arc::clone(&services.user_service));

    Schema::build(
        QueryRoot { user_query },
        MutationRoot { user_mutation },
        EmptySubscription,
    )
    .finish()
}

// スキーマビルダーを作成する関数（テスト用）
pub fn create_schema_builder() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    // Note: This is just a placeholder for testing as we can't create a proper builder with trait objects
    // In real tests, you would use a concrete implementation of UserRepository
    unimplemented!("This function should only be used with concrete types in tests")
}
