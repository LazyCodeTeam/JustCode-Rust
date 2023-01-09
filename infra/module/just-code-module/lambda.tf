module "create_profile_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "create-profile-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/create_profile_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}

module "get_profile_v1_lambda" {
  source = "../lambda-module"

  env                   = var.env
  name                  = "get-profile-v1"
  app_name              = local.app_name
  memory_size           = 128
  zip_path              = "${path.module}/../../../target/lambdas/get_profile_v1.zip"
  gateway_execution_arn = module.gateway.execution_arn
  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
  ]
}
