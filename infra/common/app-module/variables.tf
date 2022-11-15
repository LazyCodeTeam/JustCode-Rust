variable "region" {
  description = "Default region for provider"
  type        = string
  default     = "eu-central-1"
}

variable "env" {
  description = "Application env"
  type        = string
}

variable "code_service_tag" {
  description = "Code service image tag"
  type        = string
  default     = "latest"
}
