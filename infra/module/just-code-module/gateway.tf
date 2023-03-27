locals {
  swagger = templatefile("${path.module}/../../../openapi/swagger_template.yaml", {
    cognito_client_id = aws_cognito_user_pool_client.client.id
    cognito_issuer    = "https://${aws_cognito_user_pool.pool.endpoint}"
    cognito_base_url  = "https://${aws_cognito_user_pool_domain.domain.domain}.auth.${var.region}.amazoncognito.com"

    get_profile_v1           = module.get_profile_v1_lambda.invoke_arn
    update_profile_v1        = module.update_profile_v1_lambda.invoke_arn
    update_push_data_v1      = module.update_push_data_v1_lambda.invoke_arn
    remove_push_data_v1      = module.remove_push_data_v1_lambda.invoke_arn
    request_avatar_upload_v1 = module.request_avatar_upload_v1_lambda.invoke_arn

    get_public_technologies_v1 = module.get_public_technologies_v1_lambda.invoke_arn
    get_public_sections_v1     = module.get_public_sections_v1_lambda.invoke_arn
    get_public_tasks_v1        = module.get_public_tasks_v1_lambda.invoke_arn
    load_content_v1            = module.load_content_v1_lambda.invoke_arn
    load_content_dry_run_v1    = module.load_content_dry_run_v1_lambda.invoke_arn
    request_assets_upload_v1   = module.request_assets_upload_v1_lambda.invoke_arn
    get_content_assets_v1      = module.get_content_assets_v1_lambda.invoke_arn
    delete_content_assets_v1   = module.delete_content_assets_v1_lambda.invoke_arn
    answer_v1                  = module.answer_v1_lambda.invoke_arn
    get_tasks_v1               = module.get_tasks_v1_lambda.invoke_arn

    app_api_key_validator       = module.app_api_key_validator.invoke_arn
    moderator_api_key_validator = module.moderator_api_key_validator.invoke_arn
  })
}

resource "aws_apigatewayv2_api" "just_code" {
  name          = "${local.app_name}-${var.env}"
  protocol_type = "HTTP"
  body          = local.swagger

  cors_configuration {
    allow_origins = ["https://${aws_s3_bucket.swaggerui.bucket_regional_domain_name}"]
    allow_methods = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    allow_headers = ["Content-Type", "Authorization", "X-Amz-Date", "X-Api-Key", "X-Amz-Security-Token"]
  }

  tags = {
    Service     = local.app_name
    Environment = var.env
  }
}

resource "aws_apigatewayv2_stage" "just_code_api" {
  api_id      = aws_apigatewayv2_api.just_code.id
  name        = "api"
  auto_deploy = true

  access_log_settings {
    destination_arn = aws_cloudwatch_log_group.just_code_api_gw.arn

    format = jsonencode({
      requestId               = "$context.requestId"
      sourceIp                = "$context.identity.sourceIp"
      requestTime             = "$context.requestTime"
      protocol                = "$context.protocol"
      httpMethod              = "$context.httpMethod"
      resourcePath            = "$context.resourcePath"
      routeKey                = "$context.routeKey"
      status                  = "$context.status"
      responseLength          = "$context.responseLength"
      integrationErrorMessage = "$context.integrationErrorMessage"
      }
    )
  }
}


resource "aws_cloudwatch_log_group" "just_code_api_gw" {
  name = "/aws/api_gw/${aws_apigatewayv2_api.just_code.name}"

  retention_in_days = 7
}
