variable "env" {
  type = string
}

variable "app_name" {
  type = string
}

variable "name" {
  type = string
}

variable "policies_jsons" {
  type    = map(string)
  default = {}
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
  type    = list(string)
  default = []
}

variable "layers" {
  type    = list(string)
  default = []
}

variable "invoker" {
  type = object({
    arn       = string
    principal = string
  })
  default = null
}

variable "event_mapping" {
  type = object({
    event_source_arn                   = string
    enabled                            = bool
    batch_size                         = number
    maximum_batching_window_in_seconds = number
  })
  default = null
}
