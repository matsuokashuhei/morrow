#[cfg(test)]
mod tests {
    use anyhow::Result;
    use async_trait::async_trait;
    use backend::application::usecases::sign_up::SignUp;
    use backend::domain::{
        entities::{
            identity_link::{IdentityLink, NewIdentityLink},
            user::{NewUser, User},
        },
        repositories::{
            identity_link_repository::IdentityLinkRepository, user_repository::UserRepository,
        },
        services::authentication_service::AuthenticationService,
        value_objects::authentication::{Claims, SignInOutput, SignUpOutput},
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
            async fn create(&self, new_identity_link: NewIdentityLink) -> Result<IdentityLink>;
            async fn find_by_sub(&self, sub: &str) -> Result<IdentityLink>;
        }
    }

    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            identity_links: Vec::new(),
        }
    }

    fn create_test_identity_link(user_id: Uuid) -> IdentityLink {
        IdentityLink {
            id: Uuid::new_v4(),
            provider: "cognito".to_string(),
            sub: "test-sub-123".to_string(),
            user_id,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_authentication_use_case_integration() {
        // Arrange
        let mut mock_auth_service = MockAuthService::new();
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_identity_link_repo = MockIdentityLinkRepo::new();

        let test_user = create_test_user();
        let test_identity_link = create_test_identity_link(test_user.id);

        // Setup mock expectations
        mock_auth_service
            .expect_sign_up()
            .with(
                mockall::predicate::eq("test@example.com"),
                mockall::predicate::eq("password123"),
            )
            .times(1)
            .returning(|_, _| {
                Ok(SignUpOutput {
                    user_sub: "test-sub-123".to_string(),
                    user_confirmed: false,
                    session: None,
                })
            });

        mock_auth_service
            .expect_provider_name()
            .times(1)
            .returning(|| "cognito".to_string());

        let user_clone = test_user.clone();
        mock_user_repo
            .expect_create()
            .times(1)
            .returning(move |_| Ok(user_clone.clone()));

        let identity_link_clone = test_identity_link.clone();
        mock_identity_link_repo
            .expect_create()
            .times(1)
            .returning(move |_| Ok(identity_link_clone.clone()));

        // Create use cases
        let sign_up = SignUp::new(
            Arc::new(mock_auth_service),
            Arc::new(mock_user_repo),
            Arc::new(mock_identity_link_repo),
        );

        let input = backend::application::dtos::authentication_dto::SignUpInputDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = sign_up.execute(input).await;

        // Assert
        assert!(result.is_ok());
        let identity_link_dto = result.unwrap();
        assert_eq!(identity_link_dto.provider, "cognito");
        assert_eq!(identity_link_dto.sub, "test-sub-123");
        assert_eq!(identity_link_dto.user_id, test_user.id);
    }

    #[tokio::test]
    async fn test_authentication_use_case_failure() {
        // Arrange
        let mut mock_auth_service = MockAuthService::new();
        let mock_user_repo = MockUserRepo::new();
        let mock_identity_link_repo = MockIdentityLinkRepo::new();

        // Setup mock expectations for failure
        mock_auth_service
            .expect_sign_up()
            .with(
                mockall::predicate::eq("test@example.com"),
                mockall::predicate::eq("invalid_password"),
            )
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Invalid password")));

        // Create use cases
        let sign_up = SignUp::new(
            Arc::new(mock_auth_service),
            Arc::new(mock_user_repo),
            Arc::new(mock_identity_link_repo),
        );

        let input = backend::application::dtos::authentication_dto::SignUpInputDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "invalid_password".to_string(),
        };

        // Act
        let result = sign_up.execute(input).await;

        // Assert
        assert!(result.is_err());
    }
}
