use async_graphql::{Context, ErrorExtensions, Guard, Result as GraphQLResult};

use crate::domain::enums::user_role::UserRole;
use crate::presentation::graphql::context::UserContext;

// Role-based authorization guard
pub struct RoleGuard {
    pub required_role: UserRole,
}

impl RoleGuard {
    pub fn new(required_role: UserRole) -> Self {
        Self { required_role }
    }

    pub fn admin() -> Self {
        Self::new(UserRole::Admin)
    }

    pub fn user() -> Self {
        Self::new(UserRole::User)
    }
}

impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> GraphQLResult<()> {
        let user_context = ctx
            .data::<UserContext>()
            .map_err(|_| "Authentication required".extend_with(|_, e| e.set("code", "AUTH_REQUIRED")))?;

        if let Some(user) = &user_context.user {
            // Use the role from the current user context
            if has_sufficient_role(&user.role, &self.required_role) {
                Ok(())
            } else {
                Err("Insufficient permissions".extend_with(|_, e| e.set("code", "INSUFFICIENT_PERMISSIONS")))
            }
        } else {
            Err("Authentication required".extend_with(|_, e| e.set("code", "AUTH_REQUIRED")))
        }
    }
}

// Authentication guard (simpler - just checks if user is logged in)
pub struct AuthenticationGuard;

impl Guard for AuthenticationGuard {
    async fn check(&self, ctx: &Context<'_>) -> GraphQLResult<()> {
        let user_context = ctx
            .data::<UserContext>()
            .map_err(|_| "Authentication required".extend_with(|_, e| e.set("code", "AUTH_REQUIRED")))?;

        if user_context.user.is_some() {
            Ok(())
        } else {
            Err("Authentication required".extend_with(|_, e| e.set("code", "AUTH_REQUIRED")))
        }
    }
}

// Helper functions
fn has_sufficient_role(user_role: &UserRole, required_role: &UserRole) -> bool {
    use UserRole::*;

    match (user_role, required_role) {
        (Admin, _) => true,  // Admin can access everything
        (User, User) => true, // User can access user-level resources
        _ => false,
    }
}
