terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
    local = {
      source  = "hashicorp/local"
      version = "2.4.0"
    }
  }
}

locals {
  app_name          = "just-code"
  code_service_name = "code-service"
}

data "aws_ecr_repository" "code_service" {
  name = "${local.code_service_name}-${var.env}"
}
