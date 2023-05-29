terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
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

resource "aws_lambda_permission" "invoke" {
  count = var.invoker == null ? 0 : 1

  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda.function_name
  principal     = var.invoker.principal
  qualifier     = var.env
  source_arn    = var.invoker.arn
}


resource "aws_lambda_event_source_mapping" "event_source_mapping" {
  count = var.event_mapping == null ? 0 : 1

  event_source_arn                   = var.event_mapping.event_source_arn
  function_name                      = aws_lambda_function.lambda.arn
  batch_size                         = var.event_mapping.batch_size
  enabled                            = var.event_mapping.enabled
  maximum_batching_window_in_seconds = var.event_mapping.maximum_batching_window_in_seconds
}
