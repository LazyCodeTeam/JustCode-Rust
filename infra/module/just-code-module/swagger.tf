resource "local_file" "rendered_swagger" {
  content  = local.swagger
  filename = "${path.module}/../../../openapi/swagger.yaml"
}

resource "aws_s3_bucket" "swaggerui" {
  bucket = "swagger-ui-${local.app_name}-${var.env}"
}

resource "aws_s3_bucket_acl" "swaggerui" {
  bucket = aws_s3_bucket.swaggerui.id
  acl    = "private"
}

resource "aws_s3_bucket_website_configuration" "swaggerui" {
  bucket = aws_s3_bucket.swaggerui.id

  index_document {
    suffix = "index.html"
  }
}

resource "aws_s3_object" "swaggerui-index" {
  acl          = "public-read"
  bucket       = aws_s3_bucket.swaggerui.id
  key          = "index.html"
  source       = "${path.module}/../../../openapi/index.html"
  content_type = "text/html"
  etag         = filemd5("${path.module}/../../../openapi/index.html")
}

resource "aws_s3_object" "swaggerui-oauth2-redirect" {
  acl          = "public-read"
  bucket       = aws_s3_bucket.swaggerui.id
  key          = "oauth2-redirect.html"
  source       = "${path.module}/../../../openapi/oauth2-redirect.html"
  content_type = "text/html"
  etag         = filemd5("${path.module}/../../../openapi/oauth2-redirect.html")
}

resource "aws_s3_object" "swaggerui-yaml" {
  acl          = "public-read"
  bucket       = aws_s3_bucket.swaggerui.id
  key          = "swagger.yaml"
  source       = "${path.module}/../../../openapi/swagger.yaml"
  content_type = "text/yaml"
  etag         = md5(local.swagger)
  depends_on = [
    local_file.rendered_swagger
  ]
}
