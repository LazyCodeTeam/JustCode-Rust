resource "aws_s3_bucket" "images" {
  bucket = "${local.app_name}-${var.env}"
}

resource "aws_s3_bucket_acl" "images" {
  bucket = aws_s3_bucket.images.id
  acl    = "private"
}

resource "aws_s3_bucket_server_side_encryption_configuration" "images" {
  bucket = aws_s3_bucket.images.bucket

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_notification" "avatar" {
  bucket = aws_s3_bucket.images.id

  lambda_function {
    lambda_function_arn = module.on_avatar_created.arn
    events              = ["s3:ObjectCreated:Put"]
    filter_prefix       = "profile/avatar/"
  }

  depends_on = [module.on_avatar_created.permission_id]
}
