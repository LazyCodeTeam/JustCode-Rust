output "user_pool_client_secret" {
  value     = module.app.user_pool_client_secret
  sensitive = true
}

output "user_pool_client_id" {
  value     = module.app.user_pool_client_id
  sensitive = true
}
output "tasks_migration_queue_url" {
  value = module.app.tasks_migration_queue_url
}
