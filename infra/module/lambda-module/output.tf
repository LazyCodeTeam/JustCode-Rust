output "invoke_arn" {
  value = aws_lambda_alias.lambda.invoke_arn
}

output "arn" {
  value = aws_lambda_alias.lambda.arn
}

output "permission_id" {
  value = length(aws_lambda_permission.s3) == 0 ? null : aws_lambda_permission.s3[0].id
}
