use crate::application::services::Services;
use crate::presentation::graphql::mutations::user_mutation::UserMutation;
use crate::presentation::graphql::resolvers::user_resolver::UserResolver;
use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use std::sync::Arc;

use super::mutations::auth_mutation::AuthMutation;

// クエリルート定義
pub struct QueryRoot {
    user_resolver: UserResolver,
    // 他のクエリをここに追加
}

#[async_graphql::Object]
impl QueryRoot {
    // ユーザークエリへのアクセスを提供
    async fn users(&self) -> &UserResolver {
        &self.user_resolver
    }
}

// ミューテーションルート定義
pub struct MutationRoot {
    auth_mutation: AuthMutation,
    user_mutation: UserMutation,
    // 他のミューテーションをここに追加
}

#[async_graphql::Object]
impl MutationRoot {
    // ユーザーミューテーションへのアクセスを提供
    async fn users(&self) -> &UserMutation {
        &self.user_mutation
    }
    async fn auth(&self) -> &AuthMutation {
        &self.auth_mutation
    }
}

// スキーマ型エイリアス
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

// スキーマを作成する関数
pub fn create_schema(services: &Services) -> AppSchema {
    let user_resolver = UserResolver::new(Arc::clone(&services.user_service));
    let user_mutation = UserMutation::new(Arc::clone(&services.user_service));
    let auth_mutation = AuthMutation::new(Arc::clone(&services.auth_service));

    Schema::build(
        QueryRoot { user_resolver },
        MutationRoot {
            user_mutation,
            auth_mutation,
        },
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
