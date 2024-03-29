module "create_profile_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "create-profile"
  app_name      = local.app_name
  memory_size   = var.create_profile_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/create_profile.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/create_profile.json", {
      DYNAMODB_TABLE_ARN = aws_dynamodb_table.main.arn
    })
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
  memory_size = var.delete_profile_v1_memory_size
  zip_path    = "${path.module}/../../../target/lambdas/delete_profile_v1.zip"
  env_variables = merge(local.env_vars, {
    USER_POOL_ID = aws_cognito_user_pool.pool.id
  })
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/delete_profile_v1.json", {
      DYNAMODB_TABLE_ARN    = aws_dynamodb_table.main.arn
      COGNITO_USER_POOL_ARN = aws_cognito_user_pool.pool.arn
    })
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "get_profile_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "get-profile-v1"
  app_name      = local.app_name
  memory_size   = var.get_profile_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/get_profile_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/get_profile_v1.json", {
      DYNAMODB_TABLE_ARN = aws_dynamodb_table.main.arn
    })
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "update_push_data_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "update-push-data-v1"
  app_name      = local.app_name
  memory_size   = var.update_push_data_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/update_push_data_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/update_push_data_v1.json", {
      DYNAMODB_TABLE_ARN = aws_dynamodb_table.main.arn
    })
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "remove_push_data_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "remove-push-data-v1"
  app_name      = local.app_name
  memory_size   = var.remove_push_data_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/remove_push_data_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/remove_push_data_v1.json", {
      DYNAMODB_TABLE_ARN = aws_dynamodb_table.main.arn
    })
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "update_profile_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "update-profile-v1"
  app_name      = local.app_name
  memory_size   = var.update_profile_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/update_profile_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/update_profile_v1.json", {
      DYNAMODB_TABLE_ARN = aws_dynamodb_table.main.arn
    })
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "request_avatar_upload_v1_lambda" {
  source = "../lambda-module"

  env           = var.env
  name          = "request-avatar-upload-v1"
  app_name      = local.app_name
  memory_size   = var.request_avatar_upload_v1_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/request_avatar_upload_v1.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/request_avatar_upload_v1.json", {
      DYNAMODB_TABLE_ARN = aws_dynamodb_table.main.arn
      S3_BUCKET_ARN      = aws_s3_bucket.content.arn
    })
  ]
  invoker = {
    principal = "apigateway.amazonaws.com"
    arn       = "${aws_apigatewayv2_api.just_code.execution_arn}/*/*"
  }
}

module "on_avatars_created" {
  source = "../lambda-module"

  env           = var.env
  name          = "on-avatars-created"
  app_name      = local.app_name
  memory_size   = var.on_avatars_created_memory_size
  zip_path      = "${path.module}/../../../target/lambdas/on_avatars_created.zip"
  env_variables = local.env_vars
  policies_jsons = [
    templatefile("${path.module}/lambda_policies/on_avatars_created.json", {
      DYNAMODB_TABLE_ARN = aws_dynamodb_table.main.arn
      S3_BUCKET_ARN      = aws_s3_bucket.content.arn
    })
  ]
  invoker = {
    principal = "s3.amazonaws.com"
    arn       = aws_s3_bucket.content.arn
  }
}
