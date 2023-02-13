terraform {
  backend "s3" {
    bucket         = "just-code-state"
    key            = "dev/terraform.tfstate"
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

module "app" {
  source = "../module/just-code-module"

  region            = "eu-central-1"
  env               = "dev"
  moderator_api_key = var.moderator_api_key
}
