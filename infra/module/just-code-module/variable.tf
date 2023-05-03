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

variable "get_public_technologies_v1_memory_size" {
  type    = number
  default = 128
}

variable "get_public_sections_v1_memory_size" {
  type    = number
  default = 128
}

variable "get_public_tasks_v1_memory_size" {
  type    = number
  default = 128
}

variable "answer_v1_memory_size" {
  type    = number
  default = 128
}

variable "get_tasks_v1_memory_size" {
  type    = number
  default = 128
}

variable "load_content_v1_memory_size" {
  type    = number
  default = 128
}

variable "on_modifications_batch_memory_size" {
  type    = number
  default = 128
}

variable "load_content_dry_run_v1_memory_size" {
  type    = number
  default = 128
}

variable "request_assets_upload_v1_memory_size" {
  type    = number
  default = 128
}

variable "on_assets_uploaded_memory_size" {
  type    = number
  default = 128
}

variable "get_content_assets_v1_memory_size" {
  type    = number
  default = 128
}

variable "delete_content_assets_v1_memory_size" {
  type    = number
  default = 128
}

variable "moderator_api_key_validator_memory_size" {
  type    = number
  default = 128
}

variable "app_api_key_validator_memory_size" {
  type    = number
  default = 128
}
