resource "aws_cloudwatch_log_group" "service" {
  name = "${local.full_name}-logs"

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}

resource "aws_s3_bucket" "lb_logs" {
  bucket        = "${local.full_name}-lb-logs"
  force_destroy = true
}

resource "aws_s3_bucket_acl" "lb_logs" {
  bucket = aws_s3_bucket.lb_logs.id
  acl    = "private"
}

resource "aws_s3_bucket_server_side_encryption_configuration" "lb_logs" {
  bucket = aws_s3_bucket.lb_logs.bucket

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_versioning" "lb_logs" {
  bucket = aws_s3_bucket.lb_logs.id

  versioning_configuration {
    status = "Enabled"
  }

}

data "aws_elb_service_account" "main" {}

data "aws_iam_policy_document" "s3_lb_write" {
  statement {
    principals {
      identifiers = [data.aws_elb_service_account.main.arn]
      type        = "AWS"
    }

    actions = ["s3:PutObject"]

    resources = [
      "${aws_s3_bucket.lb_logs.arn}/*"
    ]
  }
}

resource "aws_s3_bucket_policy" "load_balancer_access_logs_bucket_policy" {
  bucket = aws_s3_bucket.lb_logs.id
  policy = data.aws_iam_policy_document.s3_lb_write.json
}
