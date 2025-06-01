#[cfg(test)]
mod tests {
    use backend::{
        application::{services, usecases},
        infrastructure::{
            authentication::cognito_service::CognitoService,
            database::{connection, repositories},
        },
        presentation::graphql::schema::build_schema,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn test_schema_creation() {
        // Test that we can create the GraphQL schema without errors
        // This validates that all dependencies are properly wired

        // Note: This test requires a database connection, so we'll skip it in CI
        // but it's useful for local development verification

        if std::env::var("DATABASE_URL").is_err() {
            println!("Skipping schema creation test - no DATABASE_URL set");
            return;
        }

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://scott:tiger@localhost:5432/morrow_test".to_string());

        // Try to establish database connection
        match connection::establish_connection(&database_url).await {
            Ok(conn) => {
                println!("Database connection established successfully");

                // Initialize repositories
                let repositories = repositories::init_repositories(conn);

                // Create a mock AWS config for testing
                let sdk_config =
                    aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
                let auth_service = Arc::new(CognitoService::new(&sdk_config));

                // Initialize services and use cases
                let services = services::init_services(Arc::new(repositories.clone())).await;
                let use_cases = usecases::init_use_cases(Arc::new(repositories), auth_service);

                // Try to build the GraphQL schema
                let _schema = build_schema(&use_cases, &services);
                println!("GraphQL schema created successfully");
            }
            Err(e) => {
                println!(
                    "Skipping schema creation test - database connection failed: {}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_application_structure() {
        // Simple test to verify that the application modules can be imported
        // and basic structures can be instantiated

        use backend::application::dtos::user_dto::CreateUserDto;
        use backend::domain::entities::user::NewUser;

        let create_dto = CreateUserDto {
            name: "Test User".to_string(),
        };

        let new_user = NewUser::from(create_dto);
        assert_eq!(new_user.name, "Test User");

        println!("Application structure test passed");
    }
}
