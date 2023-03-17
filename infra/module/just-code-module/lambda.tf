module "create_profile_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "create-profile"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/create_profile.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}


resource "aws_lambda_permission" "user_pool" {
  action        = "lambda:InvokeFunction"
  function_name = module.create_profile_lambda.function_name
  principal     = "cognito-idp.amazonaws.com"
  qualifier     = var.env
  source_arn    = aws_cognito_user_pool.pool.arn
}

module "get_profile_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "get-profile-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/get_profile_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}


resource "aws_lambda_permission" "get_profile_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.get_profile_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "update_push_data_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "update-push-data-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/update_push_data_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

resource "aws_lambda_permission" "update_push_data_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.update_push_data_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "remove_push_data_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "remove-push-data-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/remove_push_data_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

resource "aws_lambda_permission" "remove_push_data_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.remove_push_data_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "update_profile_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "update-profile-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/update_profile_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

resource "aws_lambda_permission" "update_profile_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.update_profile_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "request_avatar_upload_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "request-avatar-upload-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/request_avatar_upload_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
    S3_BUCKET      = aws_s3_bucket.images.id
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
}

resource "aws_lambda_permission" "request_avatar_upload_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.request_avatar_upload_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "on_avatar_created" {
  source = "../lambda-module"

  env         = var.env
  name        = "on-avatar-created"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/on_avatar_created.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
    S3_BUCKET      = aws_s3_bucket.images.id
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
}

resource "aws_lambda_permission" "on_avatar_created_s3" {
  action        = "lambda:InvokeFunction"
  function_name = module.on_avatar_created.function_name
  principal     = "s3.amazonaws.com"
  qualifier     = var.env
  source_arn    = aws_s3_bucket.images.arn
}

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
}

resource "aws_lambda_permission" "moderator_api_key_validator_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.moderator_api_key_validator.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
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
}

resource "aws_lambda_permission" "app_api_key_validator_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.app_api_key_validator.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "load_content_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "load-content-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/load_content_v1.zip"
  env_variables = {
    TASK_MIGRATION_SQS_QUEUE = aws_sqs_queue.tasks_migration.url
    DYNAMODB_TABLE           = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonSQSFullAccess",
  ]
}

resource "aws_lambda_permission" "load_content_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.load_content_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "on_modifications_batch" {
  source = "../lambda-module"

  env         = var.env
  name        = "on-modifications-batch"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/on_modifications_batch.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/service-role/AWSLambdaSQSQueueExecutionRole",
  ]
}

resource "aws_lambda_event_source_mapping" "event_source_mapping" {
  event_source_arn                   = aws_sqs_queue.tasks_migration.arn
  function_name                      = module.on_modifications_batch.arn
  batch_size                         = 25
  enabled                            = true
  maximum_batching_window_in_seconds = 20
}


module "get_public_technologies_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "get-public-technologies-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/get_public_technologies_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

resource "aws_lambda_permission" "get_public_technologies_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.get_public_technologies_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "get_public_sections_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "get-public-sections-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/get_public_sections_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

resource "aws_lambda_permission" "get_public_sections_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.get_public_sections_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}

module "get_public_tasks_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "get-public-tasks-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/get_public_tasks_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

resource "aws_lambda_permission" "get_public_tasks_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.get_public_tasks_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}


module "load_content_dry_run_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "load-content-dry-run-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/load_content_dry_run_v1.zip"
  env_variables = {
    TASK_MIGRATION_SQS_QUEUE = aws_sqs_queue.tasks_migration.url
    DYNAMODB_TABLE           = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

resource "aws_lambda_permission" "load_content_dry_run_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.load_content_dry_run_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}


module "request_assets_upload_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "request-assets-upload-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/request_assets_upload_v1.zip"
  env_variables = {
    S3_BUCKET = aws_s3_bucket.images.id
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
}

resource "aws_lambda_permission" "request_assets_upload_v1_lambda_gateway" {
  action        = "lambda:InvokeFunction"
  function_name = module.request_assets_upload_v1_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  qualifier     = var.env
  source_arn    = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
}
