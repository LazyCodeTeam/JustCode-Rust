module "gateway" {
  source = "../gateway-module"

  auth_endpoint  = aws_cognito_user_pool.pool.endpoint
  auth_client_id = aws_cognito_user_pool_client.client.id
  env            = var.env
  app_name       = local.app_name
  lambda_integrations = [
    {
      lambda_invoke_arn = module.update_profile_v1_lambda.invoke_arn
      route             = "/v1/profile/current"
      method            = "PUT"
      protected         = true
    },
    {
      lambda_invoke_arn = module.update_push_data_v1_lambda.invoke_arn
      route             = "/v1/profile/current/push"
      method            = "PUT"
      protected         = true
    },
    {
      lambda_invoke_arn = module.get_profile_v1_lambda.invoke_arn
      route             = "/v1/profile/current"
      method            = "GET"
      protected         = true
    },
    {
      lambda_invoke_arn = module.request_avatar_upload_v1_lambda.invoke_arn
      route             = "/v1/profile/current/avatar"
      method            = "POST"
      protected         = true
    },
    {
      lambda_invoke_arn = module.remove_push_data_v1_lambda.invoke_arn
      route             = "/v1/profile/current/push"
      method            = "DELETE"
      protected         = true
    },
  ]
}
