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

resource "aws_s3_bucket_notification" "just_code" {
  bucket = aws_s3_bucket.images.id

  lambda_function {
    lambda_function_arn = module.on_avatars_created.arn
    events              = ["s3:ObjectCreated:Put"]
    filter_prefix       = "profile/avatars/"
  }

  lambda_function {
    lambda_function_arn = module.on_assets_uploaded_lambda.arn
    events              = ["s3:ObjectCreated:Put"]
    filter_prefix       = "content/assets/"
  }

  depends_on = [aws_lambda_permission.on_avatars_created_s3, aws_lambda_permission.on_assets_uploaded_s3]
}
