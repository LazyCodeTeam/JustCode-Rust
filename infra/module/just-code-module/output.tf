output "user_pool_client_secret" {
  value     = aws_cognito_user_pool_client.client["mobile"].client_secret
  sensitive = true
}

output "user_pool_client_id" {
  value     = aws_cognito_user_pool_client.client["mobile"].id
  sensitive = true
}

output "tasks_migration_queue_url" {
  value = aws_sqs_queue.tasks_migration.url
}
