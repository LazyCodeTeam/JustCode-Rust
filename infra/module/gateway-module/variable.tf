variable "env" {
  type = string
}

variable "app_name" {
  type = string
}

variable "auth_endpoint" {
  type = string
}

variable "auth_client_id" {
  type = string
}

variable "moderator_authorizer_lambda_invoke_arn" {
  type = string
}

variable "lambda_integrations" {
  type = list(object({
    lambda_invoke_arn = string
    route             = string
    method            = string
    auth_type         = string
  }))
  default = []

  validation {
    condition = length([
      for o in var.lambda_integrations : true
      if contains(["COGNITO", "MODERATOR_API_KEY", "NONE"], o.auth_type)
    ]) == length(var.lambda_integrations)
    error_message = "auth_type must be one of NONE, COGNITO or MODERATOR_API_KEY"
  }
}
