module "gateway" {
  source = "../gateway-module"

  auth_endpoint  = aws_cognito_user_pool.pool.endpoint
  auth_client_id = aws_cognito_user_pool_client.client.id
  env            = var.env
  app_name       = local.app_name
  lambda_integrations = [
    {
      lambda_invoke_arn = module.create_profile_v1_lambda.invoke_arn
      route             = "/v1/profile/current"
      method            = "POST"
      protected         = true
    },
    # {
    #   lambda_invoke_arn = module.get_profile_v1_lambda.invoke_arn
    #   route             = "/v1/profile/current"
    #   method            = "GET"
    #   protected         = true
    # },
    # {
    #   lambda_invoke_arn = module.request_avatar_upload_v1_lambda.invoke_arn
    #   route             = "/v1/profile/current/avatar"
    #   method            = "POST"
    #   protected         = true
    # },
  ]
}
