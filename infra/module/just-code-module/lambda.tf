module "create_profile_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "create-profile"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/create_profile.zip"
  user_pool_arn = aws_cognito_user_pool.pool.arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.profile.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

module "get_profile_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "get-profile-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/get_profile_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.profile.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

module "update_push_data_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "update-push-data-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/update_push_data_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.profile.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

module "remove_push_data_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "remove-push-data-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/remove_push_data_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.profile.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

module "update_profile_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "update-profile-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/update_profile_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.profile.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

module "request_avatar_upload_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "request-avatar-upload-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/request_avatar_upload_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.profile.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
}

module "on_avatar_created" {
  source = "../lambda-module"

  env         = var.env
  name        = "on-avatar-created"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/on_avatar_created.zip"
  s3_arn      = aws_s3_bucket.images.arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.profile.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
}

module "moderator_api_key_validator" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "moderator-api-key-validator"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/api_key_validator.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    API_KEY = var.moderator_api_key
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
  ]
}

module "load_tasks_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "load-tasks-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/load_tasks_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    TASK_MIGRATION_SQS_QUEUE = aws_sqs_queue.tasks_migration.url
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}
