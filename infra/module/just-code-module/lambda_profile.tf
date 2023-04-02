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
  invoker = {
    principal = "cognito-idp.amazonaws.com"
    arn       = aws_cognito_user_pool.pool.arn
  }
}

module "delete_profile_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "delete-profile-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/delete_profile_v1.zip"
  env_variables = {
    DYNAMODB_TABLE = aws_dynamodb_table.main.name
    USER_POOL_ID   = aws_cognito_user_pool.pool.id
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonCognitoPowerUser"
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
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
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
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
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
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
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
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
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "request_avatar_upload_v1_lambda" {
  source = "../lambda-module"

  env         = var.env
  name        = "request-avatar-upload-v1"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/request_avatar_upload_v1.zip"
  env_variables = {
    DYNAMODB_TABLE  = aws_dynamodb_table.main.name
    S3_BUCKET       = aws_s3_bucket.content.id
    BUCKET_BASE_URL = "https://${aws_cloudfront_distribution.content.domain_name}"
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "on_avatars_created" {
  source = "../lambda-module"

  env         = var.env
  name        = "on-avatars-created"
  app_name    = local.app_name
  memory_size = 128
  zip_path    = "${path.module}/../../../target/lambdas/on_avatars_created.zip"
  env_variables = {
    DYNAMODB_TABLE  = aws_dynamodb_table.main.name
    S3_BUCKET       = aws_s3_bucket.content.id
    BUCKET_BASE_URL = "https://${aws_cloudfront_distribution.content.domain_name}"
  }
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  ]
  invoker = {
    principal = "s3.amazonaws.com"
    arn       = aws_s3_bucket.content.arn
  }
}
