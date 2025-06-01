use async_graphql::{Context, ErrorExtensions, Guard, Result as GraphQLResult};

use crate::presentation::graphql::context::UserContext;

// Role-based authorization guard
pub struct RoleGuard {
    pub required_role: UserRole,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Admin,
    SuperAdmin,
}

impl RoleGuard {
    pub fn new(required_role: UserRole) -> Self {
        Self { required_role }
    }

    pub fn admin() -> Self {
        Self::new(UserRole::Admin)
    }

    pub fn super_admin() -> Self {
        Self::new(UserRole::SuperAdmin)
    }

    pub fn user() -> Self {
        Self::new(UserRole::User)
    }
}

#[async_graphql::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> GraphQLResult<()> {
        let user_context = ctx
            .data::<UserContext>()
            .map_err(|_| "Authentication required".extend())?;

        if let Some(user) = &user_context.user {
            // For now, we'll implement basic role checking
            // In a real application, you'd store roles in the database
            let user_role = determine_user_role(&user.id).await;

            if has_sufficient_role(&user_role, &self.required_role) {
                Ok(())
            } else {
                Err("Insufficient permissions".extend())
            }
        } else {
            Err("Authentication required".extend())
        }
    }
}

// Authentication guard (simpler - just checks if user is logged in)
pub struct AuthenticationGuard;

#[async_graphql::async_trait]
impl Guard for AuthenticationGuard {
    async fn check(&self, ctx: &Context<'_>) -> GraphQLResult<()> {
        let user_context = ctx
            .data::<UserContext>()
            .map_err(|_| "Authentication required".extend())?;

        if user_context.user.is_some() {
            Ok(())
        } else {
            Err("Authentication required".extend())
        }
    }
}

// Helper functions
async fn determine_user_role(user_id: &uuid::Uuid) -> UserRole {
    // TODO: Implement actual role lookup from database
    // For now, return User role as default
    UserRole::User
}

fn has_sufficient_role(user_role: &UserRole, required_role: &UserRole) -> bool {
    use UserRole::*;

    match (user_role, required_role) {
        (SuperAdmin, _) => true,
        (Admin, Admin) | (Admin, User) => true,
        (User, User) => true,
        _ => false,
    }
}

// Resource ownership guard - checks if user owns the resource
pub struct ResourceOwnershipGuard {
    pub resource_user_id_field: String,
}

impl ResourceOwnershipGuard {
    pub fn new(resource_user_id_field: &str) -> Self {
        Self {
            resource_user_id_field: resource_user_id_field.to_string(),
        }
    }
}

#[async_graphql::async_trait]
impl Guard for ResourceOwnershipGuard {
    async fn check(&self, ctx: &Context<'_>) -> GraphQLResult<()> {
        let user_context = ctx
            .data::<UserContext>()
            .map_err(|_| "Authentication required".extend())?;

        if let Some(current_user) = &user_context.user {
            // TODO: Implement resource ownership checking
            // This would involve looking up the resource and comparing user IDs
            // For now, we'll just ensure the user is authenticated
            Ok(())
        } else {
            Err("Authentication required".extend())
        }
    }
}
