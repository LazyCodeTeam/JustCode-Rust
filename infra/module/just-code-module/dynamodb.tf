resource "aws_dynamodb_table" "main" {
  name           = "${local.app_name}-${var.env}"
  billing_mode   = var.profile_table_config.billing_mode
  read_capacity  = var.profile_table_config.read_capacity
  write_capacity = var.profile_table_config.write_capacity
  hash_key       = "PK"
  range_key      = "SK"

  attribute {
    name = "PK"
    type = "S"
  }

  attribute {
    name = "SK"
    type = "S"
  }

  attribute {
    name = "LSI_1"
    type = "S"
  }

  attribute {
    name = "LSI_2"
    type = "S"
  }

  attribute {
    name = "LSI_3"
    type = "S"
  }

  attribute {
    name = "LSI_4"
    type = "N"
  }

  attribute {
    name = "LSI_5"
    type = "N"
  }

  attribute {
    name = "GSI_1_PK"
    type = "S"
  }

  attribute {
    name = "GSI_1_SK"
    type = "S"
  }

  local_secondary_index {
    name            = "LSI_1"
    range_key       = "LSI_1"
    projection_type = "ALL"
  }

  local_secondary_index {
    name            = "LSI_2"
    range_key       = "LSI_2"
    projection_type = "ALL"
  }

  local_secondary_index {
    name            = "LSI_3"
    range_key       = "LSI_3"
    projection_type = "ALL"
  }

  local_secondary_index {
    name            = "LSI_4"
    range_key       = "LSI_4"
    projection_type = "ALL"
  }

  local_secondary_index {
    name            = "LSI_5"
    range_key       = "LSI_5"
    projection_type = "ALL"
  }

  global_secondary_index {
    name            = "GSI_1"
    hash_key        = "GSI_1_PK"
    range_key       = "GSI_1_SK"
    projection_type = "ALL"
    read_capacity   = var.profile_table_config.read_capacity
    write_capacity  = var.profile_table_config.write_capacity
  }

  ttl {
    attribute_name = "TTL"
    enabled        = true
  }

  point_in_time_recovery {
    enabled = true
  }

  tags = {
    Service     = local.app_name
    Environment = var.env
  }
}
