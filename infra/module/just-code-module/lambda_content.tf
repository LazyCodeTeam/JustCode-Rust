module "get_public_technologies_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "get-public-technologies-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/get_public_technologies_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "get_public_sections_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "get-public-sections-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/get_public_sections_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "get_public_tasks_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "get-public-tasks-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/get_public_tasks_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "answer_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "answer-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/answer_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "get_tasks_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "get-tasks-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/get_tasks_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}
