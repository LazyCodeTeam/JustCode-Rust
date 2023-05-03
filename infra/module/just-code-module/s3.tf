resource "aws_s3_bucket" "content" {
  bucket = "${local.app_name}-${var.env}"
}

resource "aws_s3_bucket_server_side_encryption_configuration" "content" {
  bucket = aws_s3_bucket.content.bucket

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_notification" "just_code" {
  bucket = aws_s3_bucket.content.id

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

  depends_on = [module.on_avatars_created, module.on_assets_uploaded_lambda]
}

data "aws_iam_policy_document" "content" {
  statement {
    actions   = ["s3:GetObject"]
    resources = ["${aws_s3_bucket.content.arn}/*"]

    principals {
      type        = "AWS"
      identifiers = [aws_cloudfront_origin_access_identity.content.iam_arn]
    }
  }
}

resource "aws_s3_bucket_policy" "content" {
  bucket = aws_s3_bucket.content.id
  policy = data.aws_iam_policy_document.content.json
}
