terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

resource "aws_iam_role" "lambda_exec" {
  name = "${var.name}-${var.env}-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
        Effect = "Allow"
        Sid    = ""
      },
    ]
  })
}

resource "aws_lambda_alias" "lambda" {
  name             = var.env
  function_name    = aws_lambda_function.lambda.arn
  function_version = aws_lambda_function.lambda.version
}

resource "aws_lambda_function" "lambda" {
  filename         = var.zip_path
  function_name    = "${var.name}-${var.env}"
  handler          = "not.required"
  source_code_hash = filebase64sha256(var.zip_path)
  runtime          = "provided.al2"
  memory_size      = var.memory_size
  role             = aws_iam_role.lambda_exec.arn
  architectures    = [var.arch]
  publish          = true
  layers           = var.layers
  environment {
    variables = merge(var.env_variables, {
      ENV = var.env
    })
  }


  tags = {
    Environment = var.env
    Name        = var.app_name
  }
}


resource "aws_iam_role_policy_attachment" "lambda_policy" {
  count = length(var.policies)

  role       = aws_iam_role.lambda_exec.name
  policy_arn = var.policies[count.index]
}

resource "aws_cloudwatch_log_group" "lambda" {
  name = "/aws/lambda/${aws_lambda_function.lambda.function_name}"

  retention_in_days = 30
}
