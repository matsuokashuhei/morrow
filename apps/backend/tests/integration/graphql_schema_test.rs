#[cfg(test)]
mod tests {
    use anyhow::Result;
    use async_graphql::{EmptySubscription, Request, Schema};
    use async_trait::async_trait;
    use backend::{
        application::{
            services::UserService,
            usecases::{
                authenticate_user::AuthenticateUser, sign_in::SignIn, sign_out::SignOut,
                sign_up::SignUp,
            },
        },
        domain::{
            entities::{
                identity_link::{IdentityLink, NewIdentityLink},
                user::{NewUser, User},
            },
            repositories::{
                identity_link_repository::IdentityLinkRepository, user_repository::UserRepository,
            },
            services::authentication_service::AuthenticationService,
            value_objects::authentication::{Claims, SignInOutput, SignUpOutput},
        },
        presentation::graphql::{
            mutations::{
                authentication_mutation::AuthenticationMutation, user_mutation::UserMutation,
            },
            resolvers::user_resolver::UserResolver,
            schema::{MutationRoot, QueryRoot},
        },
    };
    use mockall::mock;
    use std::sync::Arc;
    use uuid::Uuid;

    // Mock implementations
    mock! {
        AuthService {}

        #[async_trait]
        impl AuthenticationService for AuthService {
            fn provider_name(&self) -> String;
            async fn sign_up(&self, email: &str, password: &str) -> Result<SignUpOutput>;
            async fn sign_in(&self, email: &str, password: &str) -> Result<SignInOutput>;
            async fn sign_out(&self, access_token: &str) -> Result<()>;
            async fn verify_token(&self, access_token: &str) -> Result<Claims>;
        }
    }

    mock! {
        UserRepo {}

        #[async_trait]
        impl UserRepository for UserRepo {
            async fn create(&self, user: NewUser) -> Result<User>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
            async fn find_all(&self) -> Result<Vec<User>>;
            async fn update(&self, user: User) -> Result<User>;
            async fn delete(&self, id: Uuid) -> Result<()>;
        }
    }

    mock! {
        IdentityLinkRepo {}

        #[async_trait]
        impl IdentityLinkRepository for IdentityLinkRepo {
            async fn create(&self, identity_link: NewIdentityLink) -> Result<IdentityLink>;
            async fn find_by_sub(&self, sub: &str) -> Result<Option<IdentityLink>>;
            async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<IdentityLink>>;
            async fn delete(&self, id: Uuid) -> Result<()>;
        }
    }

    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            name: Some("Test User".to_string()),
            identity_links: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn create_test_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
        // Create mock services
        let mut mock_auth_service = MockAuthService::new();
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_identity_link_repo = MockIdentityLinkRepo::new();

        // Setup default mock behaviors
        mock_auth_service.expect_sign_up().returning(|_, _| {
            Ok(SignUpOutput {
                user_sub: "test-sub-123".to_string(),
                user_confirmed: true,
                session: None,
            })
        });

        mock_auth_service.expect_sign_in().returning(|_, _| {
            Ok(SignInOutput {
                id_token: "test-id-token".to_string(),
                access_token: "test-access-token".to_string(),
                refresh_token: "test-refresh-token".to_string(),
                expires_in: 3600,
            })
        });

        mock_auth_service.expect_sign_out().returning(|_| Ok(()));

        mock_user_repo
            .expect_create()
            .returning(|_| Ok(create_test_user()));

        mock_identity_link_repo.expect_create().returning(|_| {
            Ok(IdentityLink {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                provider: "cognito".to_string(),
                sub: "test-sub-123".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });

        mock_user_repo
            .expect_find_all()
            .returning(|| Ok(vec![create_test_user()]));

        // Create use cases
        let auth_service = Arc::new(mock_auth_service);
        let user_repo = Arc::new(mock_user_repo);
        let identity_link_repo = Arc::new(mock_identity_link_repo);

        let sign_up = Arc::new(SignUp::new(
            auth_service.clone(),
            user_repo.clone(),
            identity_link_repo.clone(),
        ));

        let sign_in = Arc::new(SignIn::new(
            auth_service.clone(),
            identity_link_repo.clone(),
            user_repo.clone(),
        ));

        let sign_out = Arc::new(SignOut::new(auth_service.clone()));

        let authenticate_user = Arc::new(AuthenticateUser::new(
            auth_service.clone(),
            identity_link_repo.clone(),
            user_repo.clone(),
        ));

        // Create services
        let user_service = Arc::new(UserService::new(user_repo.clone()));

        // Create GraphQL components
        let user_resolver = UserResolver::new(user_service.clone());
        let user_mutation = UserMutation::new(user_service.clone());
        let authentication_mutation = AuthenticationMutation::new(sign_up, sign_in, sign_out);

        // Build schema
        Schema::build(
            QueryRoot { user_resolver },
            MutationRoot {
                user_mutation,
                authentication_mutation,
            },
            EmptySubscription,
        )
        .finish()
    }

    #[tokio::test]
    async fn test_sign_up_mutation() {
        let schema = create_test_schema();

        let query = r#"
            mutation SignUp($input: SignUpInput!) {
                authentication_mutation {
                    signUp(input: $input) {
                        id
                        userId
                        provider
                        sub
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "input": {
                "name": "Test User",
                "email": "test@example.com",
                "password": "password123"
            }
        });

        let request = Request::new(query).variables(variables);
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        assert!(response.data.is_some());

        let data = response.data.unwrap();
        let auth_mutation = &data["authentication_mutation"];
        let sign_up_result = &auth_mutation["signUp"];

        assert!(sign_up_result["id"].is_string());
        assert!(sign_up_result["userId"].is_string());
        assert_eq!(sign_up_result["provider"], "cognito");
        assert_eq!(sign_up_result["sub"], "test-sub-123");
    }

    #[tokio::test]
    async fn test_sign_in_mutation() {
        let schema = create_test_schema();

        let query = r#"
            mutation SignIn($input: SignInInput!) {
                authentication_mutation {
                    signIn(input: $input) {
                        idToken
                        accessToken
                        refreshToken
                        expiresIn
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "input": {
                "email": "test@example.com",
                "password": "password123"
            }
        });

        let request = Request::new(query).variables(variables);
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        assert!(response.data.is_some());

        let data = response.data.unwrap();
        let auth_mutation = &data["authentication_mutation"];
        let sign_in_result = &auth_mutation["signIn"];

        assert_eq!(sign_in_result["idToken"], "test-id-token");
        assert_eq!(sign_in_result["accessToken"], "test-access-token");
        assert_eq!(sign_in_result["refreshToken"], "test-refresh-token");
        assert_eq!(sign_in_result["expiresIn"], 3600);
    }

    #[tokio::test]
    async fn test_users_query() {
        let schema = create_test_schema();

        let query = r#"
            query GetUsers {
                users {
                    users {
                        id
                        name
                    }
                }
            }
        "#;

        let request = Request::new(query);
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        assert!(response.data.is_some());

        let data = response.data.unwrap();
        let users_result = &data["users"]["users"];

        assert!(users_result.is_array());
        let users_array = users_result.as_array().unwrap();
        assert_eq!(users_array.len(), 1);

        let user = &users_array[0];
        assert!(user["id"].is_string());
        assert_eq!(user["name"], "Test User");
    }

    #[tokio::test]
    async fn test_invalid_sign_up_input() {
        let schema = create_test_schema();

        let query = r#"
            mutation SignUp($input: SignUpInput!) {
                authentication_mutation {
                    signUp(input: $input) {
                        id
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "input": {
                "name": "",  // Invalid: empty name
                "email": "not-an-email",  // Invalid: not a proper email
                "password": "123"  // Invalid: too short
            }
        });

        let request = Request::new(query).variables(variables);
        let response = schema.execute(request).await;

        assert!(!response.errors.is_empty());
        // The GraphQL validators should catch these validation errors
    }
}
