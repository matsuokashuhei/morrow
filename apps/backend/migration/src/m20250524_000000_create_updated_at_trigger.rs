use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // トリガー関数を作成
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION update_updated_at_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = CURRENT_TIMESTAMP;
                    RETURN NEW;
                END;
                $$ language 'plpgsql';
                "#,
            )
            .await?;

        // usersテーブルにトリガーを追加
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_users_updated_at 
                BEFORE UPDATE ON users 
                FOR EACH ROW 
                EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        // identity_linksテーブルにトリガーを追加
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_identity_links_updated_at 
                BEFORE UPDATE ON identity_links 
                FOR EACH ROW 
                EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // トリガーを削除
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS update_users_updated_at ON users")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS update_identity_links_updated_at ON identity_links")
            .await?;

        // トリガー関数を削除
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS update_updated_at_column()")
            .await?;

        Ok(())
    }
}
