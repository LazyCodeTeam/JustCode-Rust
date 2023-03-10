output "invoke_arn" {
  value = aws_lambda_alias.lambda.invoke_arn
}

output "arn" {
  value = aws_lambda_alias.lambda.arn
}

output "function_name" {
  value = aws_lambda_alias.lambda.function_name
}
