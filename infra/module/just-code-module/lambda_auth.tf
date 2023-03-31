module "moderator_api_key_validator" {
  source = "../lambda-module"

  env         = var.env
  name        = "moderator-api-key-validator"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/api_key_validator.zip"
  env_variables = {
    API_KEY = var.moderator_api_key
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "app_api_key_validator" {
  source = "../lambda-module"

  env         = var.env
  name        = "app-api-key-validator"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/api_key_validator.zip"
  env_variables = {
    API_KEY = var.app_api_key
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}
