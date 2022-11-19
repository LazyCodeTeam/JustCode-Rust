resource "aws_ecr_repository" "code_service" {
  name = "code-service-dev"

  lifecycle {
    prevent_destroy = true
  }

  tags = {
    Environment = "dev"
  }
}
