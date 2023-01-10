variable "region" {
  type = string
}

variable "env" {
  type = string
}

variable "code_service" {
  type = object({
    cpu           = number
    memory        = number
    tag           = string
    desired_count = number
  })
  default = {
    cpu           = 1024
    memory        = 2048
    tag           = "latest"
    desired_count = 1
  }
}

variable "dynamodb_billing_mode" {
  type    = string
  default = "PROVISIONED"
}

variable "dynamodb_read_capacity" {
  type    = number
  default = 1
}

variable "dynamodb_write_capacity" {
  type    = number
  default = 1
}

variable "dynamodb_gsk_read_capacity" {
  type    = number
  default = 1
}

variable "dynamodb_gsk_write_capacity" {
  type    = number
  default = 1
}
