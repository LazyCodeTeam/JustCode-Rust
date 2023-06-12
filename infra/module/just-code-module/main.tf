terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.2"
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
