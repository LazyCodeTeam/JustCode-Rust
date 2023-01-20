output "user_pool_client_secret" {
  value     = aws_cognito_user_pool_client.client.client_secret
  sensitive = true
}

output "user_pool_client_id" {
  value     = aws_cognito_user_pool_client.client.id
  sensitive = true
}
