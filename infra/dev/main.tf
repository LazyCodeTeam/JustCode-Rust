terraform {
  backend "s3" {
    bucket         = "just-code-dev-state"
    key            = "terraform.tfstate"
    region         = "eu-central-1"
    dynamodb_table = "just-code-dev-state-lock"
    encrypt        = true
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.0"
    }
  }
}

provider "aws" {
  region = var.region
}

module "backend_module" {
  source = "../common/backend-module"

  table_name  = "just-code-dev-state-lock"
  bucket_name = "just-code-dev-state"
  region      = "eu-central-1"
}
