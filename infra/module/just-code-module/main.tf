terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

locals {
  app_name = "just-code"
}


module "gateway" {
  source = "../gateway-module"

  auth_endpoint  = aws_cognito_user_pool.pool.endpoint
  auth_client_id = aws_cognito_user_pool_client.client.id
  env            = var.env
  app_name       = local.app_name
  lambda_integrations = [
    # {
    #   lambda_invoke_arn = module.get_profile_v1_lambda.invoke_arn
    #   route             = "/v1/profile/current"
    #   method            = "GET"
    #   protected         = true
    # },
    # {
    #   lambda_invoke_arn = module.create_profile_v1_lambda.invoke_arn
    #   route             = "/v1/profile/current"
    #   method            = "POST"
    #   protected         = true
    # },
    # {
    #   lambda_invoke_arn = module.request_avatar_upload_v1_lambda.invoke_arn
    #   route             = "/v1/profile/current/avatar"
    #   method            = "POST"
    #   protected         = true
    # },
  ]
}

# data "aws_ecr_repository" "code_service" {
#   name = "${local.code_service_name}-${var.env}"
# }
# module "vpc" {
#   source = "../default-vpc-module"
#
#   region = var.region
# }
#
# module "code_service" {
#   source = "../ecs-app-module"
#
#   env                = var.env
#   region             = var.region
#   service_name       = local.code_service_name
#   vpc_id             = module.vpc.vpc_id
#   public_subnet_ids  = module.vpc.public_subnet_ids
#   private_subnet_ids = module.vpc.private_subnet_ids
#   repository_tag     = var.code_service.tag
#   cpu                = var.code_service.cpu
#   memory             = var.code_service.memory
#   desired_count      = var.code_service.desired_count
#   repository_url     = data.aws_ecr_repository.code_service.repository_url
# }
