module "create_profile_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "create-profile"
  app_name      = local.app_name
  memory_size   = 128
  zip_path      = "${path.module}/../../../target/lambdas/create_profile.zip"
  user_pool_arn = aws_cognito_user_pool.pool.arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
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
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
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
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
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
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
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
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
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
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
    S3_BUCKET      = aws_s3_bucket.images.id
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
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
    S3_BUCKET      = aws_s3_bucket.images.id
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

module "app_api_key_validator" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "app-api-key-validator"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/api_key_validator.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    API_KEY = var.app_api_key
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
  ]
}

module "load_content_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "load-content-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/load_content_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
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

  env                   = var.env
  name                  = "get-public-technologies-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/get_public_technologies_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

module "get_public_sections_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "get-public-sections-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/get_public_sections_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

module "get_public_tasks_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "get-public-tasks-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/get_public_tasks_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}
