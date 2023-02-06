variable "region" {
  description = "Default region for provider"
  type        = string
  default     = "eu-central-1"
}

variable "git_personal_access_token" {
  description = "Personal access token"
  type        = string
}

variable "git_username" {
  description = "Git username"
  type        = string
}
