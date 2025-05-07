resource "aws_cognito_user_pool" "morrow" {
  name = var.aws_cognito_user_pool.morrow.name

  # Configure email as the login option
  username_attributes = ["email"]

  # # Email verification
  # verification_message_template {
  #   default_email_option = "CONFIRM_WITH_CODE"
  # }

  # Password policy
  # password_policy {
  #   minimum_length    = 8
  #   require_lowercase = true
  #   require_numbers   = true
  #   require_symbols   = true
  #   require_uppercase = true
  # }

  # Schema for user attributes
  # schema {
  #   name                = "email"
  #   attribute_data_type = "String"
  #   mutable             = true
  #   required            = true
  # }
}

resource "aws_cognito_user_pool_client" "morrow_api" {
  name                         = "morrow-api-dev"
  user_pool_id                 = aws_cognito_user_pool.morrow.id
  # generate_secret              = true
  # refresh_token_validity       = 30
  # prevent_user_existence_errors = "ENABLED"
  explicit_auth_flows = [
    "ADMIN_NO_SRP_AUTH",
    "USER_PASSWORD_AUTH",
  ]
}
