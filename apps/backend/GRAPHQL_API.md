# GraphQL API Documentation

This document provides comprehensive documentation for the Morrow Backend GraphQL API.

## Overview

The GraphQL API provides a complete interface for user management and authentication using AWS Cognito. The API is built with:

- **async-graphql**: For GraphQL schema and execution
- **Axum**: Web framework for HTTP handling
- **SeaORM**: Database ORM for PostgreSQL
- **AWS Cognito**: Authentication service

## Authentication Flow

1. **Sign Up**: Creates a new user account with AWS Cognito and stores user data locally
2. **Sign In**: Authenticates with AWS Cognito and verifies user exists locally
3. **Token Verification**: Validates JWT tokens from Cognito and fetches user data

## API Endpoints

### GraphQL Endpoint
- **URL**: `/graphql`
- **Method**: POST
- **Headers**:
  - `Content-Type: application/json`
  - `Authorization: Bearer <token>` (for authenticated requests)

### GraphQL Playground
- **URL**: `/graphql` (GET request)
- Interactive GraphQL explorer for development

### Health Check
- **URL**: `/health`
- **Method**: GET
- Returns service health status and metadata

## GraphQL Schema

### Queries

#### User Queries
```graphql
type Query {
  users: UserResolver
}

type UserResolver {
  # Get all users (admin access recommended)
  users: [User!]!

  # Get user by ID
  user(id: UUID!): User

  # Get current authenticated user's profile
  me: User

  # Get user statistics (admin access recommended)
  userStatistics: UserStatistics!
}
```

#### User Types
```graphql
type User {
  id: UUID!
  name: String!
  createdAt: DateTime!
  updatedAt: DateTime!
  identityLinks: [IdentityLink!]!
}

type UserStatistics {
  totalUsers: Int!
  activeUsers: Int!
  newUsersToday: Int!
  lastUpdated: DateTime!
}

type IdentityLink {
  id: UUID!
  provider: String!
  sub: String!
  userId: UUID!
  createdAt: DateTime!
  updatedAt: DateTime!
}
```

### Mutations

#### Authentication Mutations
```graphql
type Mutation {
  authenticationMutation: AuthenticationMutation
  users: UserMutation
}

type AuthenticationMutation {
  # Register new user account
  signUp(input: SignUpInput!): IdentityLink!

  # Sign in with email/password
  signIn(input: SignInInput!): TokenSet!

  # Sign out (revoke tokens)
  signOut(input: SignOutInput!): SignOutResponse!
}

input SignUpInput {
  name: String!
  email: String!
  password: String!
}

input SignInInput {
  email: String!
  password: String!
}

type TokenSet {
  idToken: String!
  accessToken: String!
  refreshToken: String!
  expiresIn: Int!
}
```

#### User Management Mutations
```graphql
type UserMutation {
  # Create new user (alternative to signUp)
  createUser(input: CreateUserInput!): User!

  # Update user profile
  updateUser(id: UUID!, input: UpdateUserInput!): User

  # Delete user account
  deleteUser(id: UUID!): Boolean!
}

input CreateUserInput {
  name: String!
  email: String!
  password: String!
}

input UpdateUserInput {
  name: String!
}
```

## Example Queries and Mutations

### 1. User Registration
```graphql
mutation SignUpUser {
  authenticationMutation {
    signUp(input: {
      name: "John Doe"
      email: "john@example.com"
      password: "securePassword123"
    }) {
      id
      provider
      sub
      userId
      createdAt
    }
  }
}
```

### 2. User Sign In
```graphql
mutation SignInUser {
  authenticationMutation {
    signIn(input: {
      email: "john@example.com"
      password: "securePassword123"
    }) {
      idToken
      accessToken
      refreshToken
      expiresIn
    }
  }
}
```

### 3. Get Current User Profile
```graphql
query GetMyProfile {
  users {
    me {
      id
      name
      createdAt
      updatedAt
      identityLinks {
        provider
        sub
        createdAt
      }
    }
  }
}
```

### 4. Get All Users (Admin)
```graphql
query GetAllUsers {
  users {
    users {
      id
      name
      createdAt
      identityLinks {
        provider
        sub
      }
    }
  }
}
```

### 5. Get User Statistics (Admin)
```graphql
query GetUserStats {
  users {
    userStatistics {
      totalUsers
      activeUsers
      newUsersToday
      lastUpdated
    }
  }
}
```

### 6. Update User Profile
```graphql
mutation UpdateUserProfile {
  users {
    updateUser(
      id: "123e4567-e89b-12d3-a456-426614174000"
      input: {
        name: "John Smith"
      }
    ) {
      id
      name
      updatedAt
    }
  }
}
```

## Error Handling

The API returns GraphQL-compliant error responses:

```json
{
  "errors": [
    {
      "message": "Authentication required",
      "locations": [{"line": 2, "column": 3}],
      "path": ["users", "me"]
    }
  ],
  "data": null
}
```

Common error scenarios:
- **Authentication required**: User must be signed in
- **Insufficient permissions**: User lacks required role/permissions
- **Validation errors**: Input validation failures
- **Not found**: Requested resource doesn't exist
- **Service errors**: External service (Cognito) failures

## Security Considerations

1. **Authentication**: JWT tokens from AWS Cognito are validated
2. **Authorization**: Role-based access control (implementation ready)
3. **Input validation**: GraphQL input validation with constraints
4. **Rate limiting**: Should be implemented at infrastructure level
5. **CORS**: Configure appropriately for frontend domains

## Development

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test file
cargo test graphql_schema_test

# Run with output
cargo test -- --nocapture
```

### Schema Introspection
Use the GraphQL Playground at `/graphql` to explore the schema interactively.

### Adding New Features
1. Define domain entities in `src/domain/entities/`
2. Create repository traits in `src/domain/repositories/`
3. Implement repositories in `src/infrastructure/database/repositories/`
4. Add DTOs in `src/application/dtos/`
5. Create use cases in `src/application/usecases/`
6. Add GraphQL types in `src/presentation/graphql/types/`
7. Create resolvers/mutations in `src/presentation/graphql/`
8. Update schema in `src/presentation/graphql/schema.rs`
9. Add tests in `tests/`

This architecture follows clean architecture principles with clear separation of concerns.
