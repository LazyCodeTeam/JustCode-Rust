variable "env" {
  type = string
}

variable "app_name" {
  type = string
}

variable "name" {
  type = string
}

variable "env_variables" {
  type    = map(string)
  default = {}
}

variable "memory_size" {
  type = number
}

variable "zip_path" {
  type = string
}

variable "arch" {
  type    = string
  default = "arm64"
}

variable "policies" {
  type = list(string)
}

variable "layers" {
  type    = list(string)
  default = []
}
