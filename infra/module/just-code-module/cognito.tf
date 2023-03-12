resource "aws_cognito_user_pool" "pool" {
  name = "${local.app_name}-${var.env}-pool"

  auto_verified_attributes = ["email"]
  alias_attributes         = ["email"]

  account_recovery_setting {
    recovery_mechanism {
      name     = "verified_email"
      priority = 1
    }
  }

  lambda_config {
    post_confirmation = module.create_profile_lambda.arn
  }

  password_policy {
    minimum_length                   = 8
    temporary_password_validity_days = 7
  }

  username_configuration {
    case_sensitive = false
  }

  schema {
    attribute_data_type      = "String"
    developer_only_attribute = false
    mutable                  = false
    name                     = "email"
    required                 = true

    string_attribute_constraints {
      min_length = 7
      max_length = 256
    }
  }

  tags = {
    AppName = local.app_name
    Env     = var.env
  }
}

resource "aws_cognito_user_pool_client" "client" {
  name                                 = "${local.app_name}-${var.env}-client"
  user_pool_id                         = aws_cognito_user_pool.pool.id
  generate_secret                      = true
  allowed_oauth_flows_user_pool_client = true
  prevent_user_existence_errors        = "ENABLED"
  access_token_validity                = 15
  refresh_token_validity               = 30
  allowed_oauth_flows                  = ["code"]
  callback_urls                        = ["https://${aws_s3_bucket.swaggerui.bucket_regional_domain_name}/oauth2-redirect.html", "https://example.com/"]
  logout_urls                          = ["https://example.com/"]

  allowed_oauth_scopes = [
    "aws.cognito.signin.user.admin",
    "email",
    "openid",
    "phone",
    "profile",
  ]

  token_validity_units {
    access_token  = "minutes"
    refresh_token = "days"
  }

  supported_identity_providers = [
    "COGNITO",
  ]

  explicit_auth_flows = [
    "ALLOW_REFRESH_TOKEN_AUTH",
    "ALLOW_USER_PASSWORD_AUTH",
  ]
}

resource "aws_cognito_user_pool_domain" "domain" {
  domain       = "${local.app_name}-${var.env}"
  user_pool_id = aws_cognito_user_pool.pool.id
}
