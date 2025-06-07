use anyhow::Result;
use std::sync::Arc;

use crate::{
    application::dtos::{authentication_dto::SignUpInputDTO, identity_link_dto::IdentityLinkDto},
    domain::{
        entities::{identity_link::NewIdentityLink, user::NewUser},
        repositories::{
            identity_link_repository::IdentityLinkRepository, user_repository::UserRepository,
        },
        services::authentication_service::AuthenticationService,
    },
};

pub struct SignUp {
    authentication_service: Arc<dyn AuthenticationService>,
    user_repository: Arc<dyn UserRepository>,
    identity_link_repository: Arc<dyn IdentityLinkRepository>,
}

impl SignUp {
    pub fn new(
        authentication_service: Arc<dyn AuthenticationService>,
        user_repository: Arc<dyn UserRepository>,
        identity_link_repository: Arc<dyn IdentityLinkRepository>,
    ) -> Self {
        Self {
            authentication_service,
            user_repository,
            identity_link_repository,
        }
    }

    pub async fn execute(&self, input: SignUpInputDTO) -> Result<IdentityLinkDto> {
        let output = self
            .authentication_service
            .sign_up(&input.email, &input.password)
            .await?;
        let user = self.user_repository.create(NewUser::from(input)).await?;
        let identity_link = self
            .identity_link_repository
            .create(NewIdentityLink {
                provider: self.authentication_service.provider_name(),
                sub: output.user_sub,
                user_id: user.id,
            })
            .await?;
        Ok(IdentityLinkDto::from(identity_link))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dtos::{
        authentication_dto::SignUpInputDTO,
    };
    use crate::domain::{
        entities::{identity_link::IdentityLink, user::User},
        repositories::{
            identity_link_repository::IdentityLinkRepository, user_repository::UserRepository,
        },
        services::authentication_service::AuthenticationService,
        value_objects::authentication::SignUpOutput,
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
            async fn sign_in(&self, email: &str, password: &str) -> Result<crate::domain::value_objects::authentication::SignInOutput>;
            async fn sign_out(&self, access_token: &str) -> Result<()>;
            async fn verify_token(&self, access_token: &str) -> Result<crate::domain::value_objects::authentication::Claims>;
        }
    }

    mock! {
        UserRepo {}

        #[async_trait]
        impl UserRepository for UserRepo {
            async fn create(&self, user: crate::domain::entities::user::NewUser) -> Result<User>;
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
            async fn create(&self, new_identity_link: crate::domain::entities::identity_link::NewIdentityLink) -> Result<IdentityLink>;
            async fn find_by_sub(&self, sub: &str) -> Result<IdentityLink>;
        }
    }

    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            role: crate::domain::enums::user_role::UserRole::User,
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
    async fn test_sign_up_success() {
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

        // Create sign_up use case
        let sign_up = SignUp::new(
            Arc::new(mock_auth_service),
            Arc::new(mock_user_repo),
            Arc::new(mock_identity_link_repo),
        );

        let input = SignUpInputDTO {
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
    async fn test_sign_up_authentication_service_failure() {
        // Arrange
        let mut mock_auth_service = MockAuthService::new();
        let mock_user_repo = MockUserRepo::new();
        let mock_identity_link_repo = MockIdentityLinkRepo::new();

        // Setup mock expectations for authentication failure
        mock_auth_service
            .expect_sign_up()
            .with(
                mockall::predicate::eq("test@example.com"),
                mockall::predicate::eq("password123"),
            )
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Authentication service error")));

        // Create sign_up use case
        let sign_up = SignUp::new(
            Arc::new(mock_auth_service),
            Arc::new(mock_user_repo),
            Arc::new(mock_identity_link_repo),
        );

        let input = SignUpInputDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = sign_up.execute(input).await;

        // Assert
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Authentication service error"));
    }

    #[tokio::test]
    async fn test_sign_up_user_repository_failure() {
        // Arrange
        let mut mock_auth_service = MockAuthService::new();
        let mut mock_user_repo = MockUserRepo::new();
        let mock_identity_link_repo = MockIdentityLinkRepo::new();

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

        // Setup user repository failure
        mock_user_repo
            .expect_create()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        // Create sign_up use case
        let sign_up = SignUp::new(
            Arc::new(mock_auth_service),
            Arc::new(mock_user_repo),
            Arc::new(mock_identity_link_repo),
        );

        let input = SignUpInputDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = sign_up.execute(input).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_sign_up_identity_link_repository_failure() {
        // Arrange
        let mut mock_auth_service = MockAuthService::new();
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_identity_link_repo = MockIdentityLinkRepo::new();

        let test_user = create_test_user();

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

        // Setup identity link repository failure
        mock_identity_link_repo
            .expect_create()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Identity link creation failed")));

        // Create sign_up use case
        let sign_up = SignUp::new(
            Arc::new(mock_auth_service),
            Arc::new(mock_user_repo),
            Arc::new(mock_identity_link_repo),
        );

        let input = SignUpInputDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = sign_up.execute(input).await;

        // Assert
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Identity link creation failed"));
    }
}
