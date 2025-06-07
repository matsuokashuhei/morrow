// filepath: /workspace/morrow/apps/backend/tests/authorization_test.rs
    use async_graphql::{Schema, EmptyMutation, EmptySubscription, Object};
    use backend::{
        application::dtos::user_dto::UserDTO,
        domain::enums::user_role::UserRole,
        presentation::graphql::{
            context::UserContext,
            guards::{AuthenticationGuard, RoleGuard},
        },
    };
    use chrono::Utc;
    use uuid::Uuid;

    struct QueryRoot;

    #[Object]
    impl QueryRoot {
        #[graphql(guard = "AuthenticationGuard")]
        async fn protected_query(&self) -> String {
            "This is protected content".to_string()
        }

        #[graphql(guard = "RoleGuard::admin()")]
        async fn admin_only_query(&self) -> String {
            "This is admin only content".to_string()
        }
    }

    type TestSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

    fn create_test_schema() -> TestSchema {
        Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
    }

    fn create_user_context(role: UserRole) -> UserContext {
        UserContext {
            user: Some(UserDTO {
                id: Uuid::new_v4(),
                name: "Test User".to_string(),
                role,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                identity_links: Vec::new(),
            }),
        }
    }

    #[tokio::test]
    async fn test_authentication_guard_with_user() {
        let schema = create_test_schema();
        let user_context = create_user_context(UserRole::User);

        let request = async_graphql::Request::new("{ protectedQuery }");
        let response = schema
            .execute(request.data(user_context))
            .await;

        assert!(response.errors.is_empty());
        assert_eq!(
            response.data.to_string(),
            r#"{protectedQuery: "This is protected content"}"#
        );
    }

    #[tokio::test]
    async fn test_authentication_guard_without_user() {
        let schema = create_test_schema();
        let user_context = UserContext { user: None };

        let request = async_graphql::Request::new("{ protectedQuery }");
        let response = schema
            .execute(request.data(user_context))
            .await;

        assert!(!response.errors.is_empty());
        assert!(response.errors[0].message.contains("Authentication required"));
    }

    #[tokio::test]
    async fn test_role_guard_admin_access() {
        let schema = create_test_schema();
        let admin_context = create_user_context(UserRole::Admin);

        let request = async_graphql::Request::new("{ adminOnlyQuery }");
        let response = schema
            .execute(request.data(admin_context))
            .await;

        assert!(response.errors.is_empty());
        assert_eq!(
            response.data.to_string(),
            r#"{adminOnlyQuery: "This is admin only content"}"#
        );
    }

    #[tokio::test]
    async fn test_role_guard_user_denied() {
        let schema = create_test_schema();
        let user_context = create_user_context(UserRole::User);

        let request = async_graphql::Request::new("{ adminOnlyQuery }");
        let response = schema
            .execute(request.data(user_context))
            .await;

        assert!(!response.errors.is_empty());
        assert!(response.errors[0].message.contains("Insufficient permissions"));
    }
