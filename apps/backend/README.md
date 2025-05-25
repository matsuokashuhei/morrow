[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/matsuokashuhei/morrow)

# Backend Application

## Overview

This is a GraphQL API application built with Rust using:
- Axum (Web framework)
- SeaORM (ORM for PostgreSQL)
- async-graphql (GraphQL implementation)
- AWS Cognito (Authentication)

## Database Configuration

This application uses PostgreSQL as its database. The following components have been configured for PostgreSQL:

### Required Environment Variables

```env
# PostgreSQL Configuration
POSTGRES_USER=scott
POSTGRES_PASSWORD=tiger
POSTGRES_DB=morrow
POSTGRES_HOST=postgres
POSTGRES_PORT=5432

# AWS Cognito Configuration
AWS_COGNITO_USER_POOL_ID=xxx
AWS_COGNITO_USER_POOL_CLIENT_ID=xxx

# Application Configuration
HOST=0.0.0.0
PORT=3000
```

### Database Components

1. **SeaORM Configuration**: Uses `sqlx-postgres` driver with `runtime-tokio-rustls`
2. **Migrations**: All migration files support PostgreSQL syntax including triggers
3. **Database Connection**: PostgreSQL connection string format
4. **SchemaSpy**: Configured for PostgreSQL documentation generation

## Running the Application

### Prerequisites

1. Docker and Docker Compose
2. Rust toolchain (for local development)
3. AWS Cognito User Pool Client ID

### Setup Steps

```bash
# 1. Update the .env file with your AWS Cognito Client ID
# Edit /Users/matsuokashuhei/Development/morrow/apps/.env
# Set AWS_COGNITO_USER_POOL_CLIENT_ID to your actual value

# 2. Start the PostgreSQL database and related services
docker compose up -d postgres

# 3. Run database migrations
cd backend
cargo run --bin migration

# 4. Start the backend application
cargo run

# Or start everything with docker compose
docker compose up -d
```

### Development Commands

```bash
# Generate new migration
sea-orm-cli migrate generate create_new_table

# Run migrations
cargo run --bin migration

# Generate entities from database
sea-orm-cli generate entity -o src/infrastructure/database/models

# View database documentation (after starting schemaspy service)
# http://localhost:8080
```

### API Endpoints

- **GraphQL Playground**: http://localhost:3000/graphql
- **Health Check**: http://localhost:3000/health
- **Database Documentation**: http://localhost:8080 (when schemaspy service is running)

## Migration Notes

The application has been fully migrated from MySQL to PostgreSQL with the following changes:

1. **Cargo.toml**: Updated SeaORM features to use `sqlx-postgres`
2. **Database URL**: Changed to PostgreSQL format in configuration
3. **Migrations**: Added PostgreSQL-specific trigger functions for `updated_at` columns
4. **Docker Compose**: PostgreSQL service configuration
5. **SchemaSpy**: PostgreSQL driver configuration

All PostgreSQL-specific features like triggers and functions are properly implemented in the migration files.
