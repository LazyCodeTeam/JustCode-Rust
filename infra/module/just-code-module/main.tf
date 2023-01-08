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
