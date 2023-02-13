variable "region" {
  description = "Default region for provider"
  type        = string
  default     = "eu-central-1"
}

variable "moderator_api_key" {
  type      = string
  sensitive = true
  default   = ""
}
