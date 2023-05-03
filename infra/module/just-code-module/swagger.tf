locals {
  swaggerui_content = replace(local.swagger, "$${base_url}", local.gateway_base_url)
}
resource "local_file" "rendered_swagger" {
  content  = local.swaggerui_content
  filename = "${path.module}/../../../openapi/swagger.yaml"
}

resource "aws_s3_bucket" "swaggerui" {
  bucket = "swagger-ui-${local.app_name}-${var.env}"
}

resource "aws_s3_bucket_website_configuration" "swaggerui" {
  bucket = aws_s3_bucket.swaggerui.id

  index_document {
    suffix = "index.html"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "swaggerui" {
  bucket = aws_s3_bucket.swaggerui.bucket

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_object" "swaggerui-index" {
  bucket       = aws_s3_bucket.swaggerui.id
  key          = "index.html"
  source       = "${path.module}/../../../openapi/index.html"
  content_type = "text/html"
  etag         = filemd5("${path.module}/../../../openapi/index.html")
}

resource "aws_s3_object" "swaggerui-oauth2-redirect" {
  bucket       = aws_s3_bucket.swaggerui.id
  key          = "oauth2-redirect.html"
  source       = "${path.module}/../../../openapi/oauth2-redirect.html"
  content_type = "text/html"
  etag         = filemd5("${path.module}/../../../openapi/oauth2-redirect.html")
}

resource "aws_s3_object" "swaggerui-yaml" {
  bucket       = aws_s3_bucket.swaggerui.id
  key          = "swagger.yaml"
  source       = "${path.module}/../../../openapi/swagger.yaml"
  content_type = "text/yaml"
  etag         = md5(local.swaggerui_content)
  depends_on = [
    local_file.rendered_swagger
  ]
}

resource "aws_s3_bucket_policy" "swaggerui" {
  bucket = aws_s3_bucket.swaggerui.id

  policy = data.aws_iam_policy_document.swaggerui.json
}

data "aws_iam_policy_document" "swaggerui" {
  statement {
    sid     = "PublicReadGetObjectSwaggerUI"
    actions = ["s3:GetObject"]
    principals {
      type        = "*"
      identifiers = ["*"]
    }
    resources = [
      aws_s3_bucket.swaggerui.arn,
      "${aws_s3_bucket.swaggerui.arn}/*",
    ]
  }
}
