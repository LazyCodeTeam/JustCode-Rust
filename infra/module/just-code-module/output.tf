output "user_pool_client_secret" {
  value     = aws_cognito_user_pool_client.client.client_secret
  sensitive = true
}
