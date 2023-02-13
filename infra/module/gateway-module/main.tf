terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

locals {
  authorizer_type = {
    NONE              = null,
    COGNITO           = "JWT",
    MODERATOR_API_KEY = "CUSTOM"
  }
  authorizer_id = {
    NONE              = null,
    COGNITO           = aws_apigatewayv2_authorizer.auth.id,
    MODERATOR_API_KEY = aws_apigatewayv2_authorizer.moderator.id
  }
}

resource "aws_apigatewayv2_api" "default" {
  name          = "${var.app_name}-${var.env}"
  protocol_type = "HTTP"

  tags = {
    Service     = var.app_name
    Environment = var.env
  }
}

resource "aws_apigatewayv2_stage" "default" {
  api_id      = aws_apigatewayv2_api.default.id
  name        = "api"
  auto_deploy = true

  access_log_settings {
    destination_arn = aws_cloudwatch_log_group.api_gw.arn

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

resource "aws_apigatewayv2_authorizer" "auth" {
  api_id           = aws_apigatewayv2_api.default.id
  authorizer_type  = "JWT"
  identity_sources = ["$request.header.Authorization"]
  name             = "${var.app_name}-${var.env}-auth"

  jwt_configuration {
    audience = [var.auth_client_id]
    issuer   = "https://${var.auth_endpoint}"
  }
}


resource "aws_apigatewayv2_integration" "lambdas" {
  count = length(var.lambda_integrations)

  api_id             = aws_apigatewayv2_api.default.id
  integration_type   = "AWS_PROXY"
  integration_method = "POST"
  integration_uri    = var.lambda_integrations[count.index].lambda_invoke_arn
}

resource "aws_apigatewayv2_route" "lambdas" {
  count = length(var.lambda_integrations)

  api_id             = aws_apigatewayv2_api.default.id
  route_key          = "${var.lambda_integrations[count.index].method} ${var.lambda_integrations[count.index].route}"
  authorizer_id      = local.authorizer_id[var.lambda_integrations[count.index].auth_type]
  authorization_type = local.authorizer_type[var.lambda_integrations[count.index].auth_type]

  target = "integrations/${aws_apigatewayv2_integration.lambdas[count.index].id}"
}

resource "aws_cloudwatch_log_group" "api_gw" {
  name = "/aws/api_gw/${aws_apigatewayv2_api.default.name}"

  retention_in_days = 30
}

resource "aws_apigatewayv2_authorizer" "moderator" {
  api_id                            = aws_apigatewayv2_api.default.id
  authorizer_type                   = "REQUEST"
  authorizer_uri                    = var.moderator_authorizer_lambda_invoke_arn
  identity_sources                  = ["$request.header.X-Api-Key"]
  name                              = "${var.app_name}-${var.env}-moderator-authorizer"
  authorizer_payload_format_version = "2.0"
  enable_simple_responses           = true
}
