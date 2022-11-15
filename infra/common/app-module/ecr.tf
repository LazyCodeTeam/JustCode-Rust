resource "aws_ecr_repository" "code_service" {
  name = "code-service-${var.env}"
  tags = {
    Environment = var.env
  }
}
