terraform {
  backend "s3" {
    bucket         = "just-code-state"
    key            = "base/terraform.tfstate"
    region         = "eu-central-1"
    dynamodb_table = "just-code-state-lock"
    encrypt        = true
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region = var.region
}

locals {
  bucket_name = "just-code-state"
  table_name  = "just-code-state-lock"
}
