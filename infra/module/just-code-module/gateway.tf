module "gateway" {
  source = "../gateway-module"

  auth_endpoint                          = aws_cognito_user_pool.pool.endpoint
  auth_client_id                         = aws_cognito_user_pool_client.client.id
  env                                    = var.env
  app_name                               = local.app_name
  moderator_authorizer_lambda_invoke_arn = module.moderator_api_key_validator.invoke_arn
  app_authorizer_lambda_invoke_arn       = module.app_api_key_validator.invoke_arn
  lambda_integrations = [
    {
      lambda_invoke_arn = module.update_profile_v1_lambda.invoke_arn
      route             = "/v1/profile/current"
      method            = "PUT"
      auth_type         = "COGNITO"
    },
    {
      lambda_invoke_arn = module.update_push_data_v1_lambda.invoke_arn
      route             = "/v1/profile/current/push"
      method            = "PUT"
      auth_type         = "COGNITO"
    },
    {
      lambda_invoke_arn = module.get_profile_v1_lambda.invoke_arn
      route             = "/v1/profile/current"
      method            = "GET"
      auth_type         = "COGNITO"
    },
    {
      lambda_invoke_arn = module.request_avatar_upload_v1_lambda.invoke_arn
      route             = "/v1/profile/current/avatar"
      method            = "POST"
      auth_type         = "COGNITO"
    },
    {
      lambda_invoke_arn = module.remove_push_data_v1_lambda.invoke_arn
      route             = "/v1/profile/current/push"
      method            = "DELETE"
      auth_type         = "COGNITO"
    },
    {
      lambda_invoke_arn = module.load_content_v1_lambda.invoke_arn
      route             = "/v1/content/load"
      method            = "PUT"
      auth_type         = "MODERATOR_API_KEY"
    },
    {
      lambda_invoke_arn = module.get_public_technologies_v1_lambda.invoke_arn
      route             = "/v1/content/public/technologies"
      method            = "GET"
      auth_type         = "APP_API_KEY"
    },
    {
      lambda_invoke_arn = module.get_public_sections_v1_lambda.invoke_arn
      route             = "/v1/content/public/technology/{technology_id}/sections"
      method            = "GET"
      auth_type         = "APP_API_KEY"
    },
    {
      lambda_invoke_arn = module.get_public_tasks_v1_lambda.invoke_arn
      route             = "/v1/content/public/section/{section_id}/tasks"
      method            = "GET"
      auth_type         = "APP_API_KEY"
    },
  ]
}
