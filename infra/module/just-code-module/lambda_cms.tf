module "load_content_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "load-content-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/load_content_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonSQSFullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "on_modifications_batch" {
  source = "../lambda-module"

  env           = var.env
  name          = "on-modifications-batch"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/on_modifications_batch.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/service-role/AWSLambdaSQSQueueExecutionRole",
  ]
  event_mapping = {
    event_source_arn                   = aws_sqs_queue.tasks_migration.arn
    batch_size                         = 25
    enabled                            = true
    maximum_batching_window_in_seconds = 20
  }
}

module "load_content_dry_run_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "load-content-dry-run-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/load_content_dry_run_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "request_assets_upload_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "request-assets-upload-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/request_assets_upload_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "on_assets_uploaded_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "on-assets-uploaded"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/on_assets_uploaded.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
  invoker = {
    principal = "s3.amazonaws.com"
    arn       = aws_s3_bucket.content.arn
  }
}

module "get_content_assets_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "get-content-assets-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/get_content_assets_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "delete_content_assets_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "delete-content-assets-v1"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/delete_content_assets_v1.zip"
  env_variables = local.env_vars
  policies = [
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}
