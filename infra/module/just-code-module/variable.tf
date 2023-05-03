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
    cpu           = 2048
    memory        = 4096
    tag           = "latest"
    desired_count = 1
  }
}

variable "main_table_config" {
  type = object({
    billing_mode   = string
    read_capacity  = number
    write_capacity = number
  })
  default = {
    billing_mode   = "PAY_PER_REQUEST"
    read_capacity  = null
    write_capacity = null
  }
}

variable "content_table_config" {
  type = object({
    billing_mode   = string
    read_capacity  = number
    write_capacity = number
  })
  default = {
    billing_mode   = "PAY_PER_REQUEST"
    read_capacity  = null
    write_capacity = null
  }
}

variable "moderator_api_key" {
  type = string
}

variable "app_api_key" {
  type = string
}

variable "create_profile_memory_size" {
  type    = number
  default = 128
}

variable "delete_profile_v1_memory_size" {
  type    = number
  default = 128
}

variable "get_profile_v1_memory_size" {
  type    = number
  default = 128
}

variable "update_push_data_v1_memory_size" {
  type    = number
  default = 128
}

variable "remove_push_data_v1_memory_size" {
  type    = number
  default = 128
}

variable "update_profile_v1_memory_size" {
  type    = number
  default = 128
}

variable "request_avatar_upload_v1_memory_size" {
  type    = number
  default = 128
}

variable "on_avatars_created_memory_size" {
  type    = number
  default = 128
}
