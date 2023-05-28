module "get_public_technologies_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "get-public-technologies-v1"
  app_name      = local.app_name
  memory_size   = var.get_public_technologies_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/get_public_technologies_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/get_public_technologies_v1.json", {
      CONTENT_DYNAMODB_TABLE_ARN = aws_dynamodb_table.content.arn
    })
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
  memory_size   = var.get_public_sections_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/get_public_sections_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/get_public_sections_v1.json", {
      CONTENT_DYNAMODB_TABLE_ARN = aws_dynamodb_table.content.arn
    })
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
  memory_size   = var.get_public_tasks_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/get_public_tasks_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/get_public_tasks_v1.json", {
      CONTENT_DYNAMODB_TABLE_ARN = aws_dynamodb_table.content.arn
    })
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
  memory_size   = var.answer_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/answer_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/answer_v1.json", {
      CONTENT_DYNAMODB_TABLE_ARN = aws_dynamodb_table.content.arn
      DYNAMODB_TABLE_ARN         = aws_dynamodb_table.main.arn
    })
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
  memory_size   = var.get_tasks_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/get_tasks_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/get_tasks_v1.json", {
      CONTENT_DYNAMODB_TABLE_ARN = aws_dynamodb_table.content.arn
      DYNAMODB_TABLE_ARN         = aws_dynamodb_table.main.arn
    })
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "get_sections_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "get-sections-v1"
  app_name      = local.app_name
  memory_size   = var.get_tasks_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/get_sections_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/get_sections_v1.json", {
      CONTENT_DYNAMODB_TABLE_ARN = aws_dynamodb_table.content.arn
      DYNAMODB_TABLE_ARN         = aws_dynamodb_table.main.arn
    })
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}
