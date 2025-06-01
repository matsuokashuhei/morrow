#[cfg(test)]
mod tests {
    use async_graphql::{Request, Schema, EmptySubscription};
    use backend::{
        application::{
            usecases::{sign_up::SignUp, sign_in::SignIn, sign_out::SignOut},
            services::UserService,
        },
        domain::{
            entities::{identity_link::{IdentityLink, NewIdentityLink}, user::{User, NewUser}},
            repositories::{identity_link_repository::IdentityLinkRepository, user_repository::UserRepository},
            services::authentication_service::AuthenticationService,
            value_objects::authentication::{SignUpOutput, SignInOutput, Claims},
        },
        presentation::graphql::{
            schema::{QueryRoot, MutationRoot},
            mutations::{authentication_mutation::AuthenticationMutation, user_mutation::UserMutation},
            resolvers::user_resolver::UserResolver,
        },
    };
    use anyhow::Result;
    use async_trait::async_trait;
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
        mock_auth_service
            .expect_sign_up()
            .returning(|_, _| {
                Ok(SignUpOutput {
                    user_sub: "test-sub-123".to_string(),
                    user_confirmed: true,
                    session: None,
                })
            });

        mock_auth_service
            .expect_sign_in()
            .returning(|_, _| {
                Ok(SignInOutput {
                    id_token: "test-id-token".to_string(),
                    access_token: "test-access-token".to_string(),
                    refresh_token: "test-refresh-token".to_string(),
                    expires_in: 3600,
                })
            });

        mock_auth_service
            .expect_sign_out()
            .returning(|_| Ok(()));

        mock_user_repo
            .expect_create()
            .returning(|_| Ok(create_test_user()));

        mock_identity_link_repo
            .expect_create()
            .returning(|_| {
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

        // Create services
        let user_service = Arc::new(UserService::new(user_repo.clone()));

        // Create GraphQL components
        let user_resolver = UserResolver::new(user_service.clone());
        let user_mutation = UserMutation::new(user_service.clone());
        let authentication_mutation = AuthenticationMutation::new(
            sign_up,
            sign_in,
            sign_out,
        );

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

        println!("Response: {:?}", response);
        assert!(response.errors.is_empty(), "GraphQL errors: {:?}", response.errors);
        assert!(response.data.is_some());
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

        println!("Response: {:?}", response);
        assert!(response.errors.is_empty(), "GraphQL errors: {:?}", response.errors);
        assert!(response.data.is_some());
    }
}
