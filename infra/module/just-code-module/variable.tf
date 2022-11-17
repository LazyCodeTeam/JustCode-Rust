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
    cpu           = 4096
    memory        = 8192
    tag           = "latest"
    desired_count = 1
  }
}
