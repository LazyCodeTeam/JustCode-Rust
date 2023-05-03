locals {
  s3_origin_id = "s3-${local.app_name}-${var.env}-content-origin"
}

resource "aws_cloudfront_cache_policy" "content" {
  name = "${local.app_name}-${var.env}-content-cache-policy"

  min_ttl     = 0
  default_ttl = 3600
  max_ttl     = 86400

  parameters_in_cache_key_and_forwarded_to_origin {
    cookies_config {
      cookie_behavior = "none"
    }

    headers_config {
      header_behavior = "none"
    }

    query_strings_config {
      query_string_behavior = "none"
    }
  }
}


resource "aws_cloudfront_origin_access_identity" "content" {
  comment = "Content access identity for ${local.app_name}-${var.env}"
}

resource "aws_cloudfront_distribution" "content" {
  enabled         = true
  is_ipv6_enabled = true
  price_class     = "PriceClass_200"

  origin {
    origin_id   = local.s3_origin_id
    domain_name = aws_s3_bucket.content.bucket_regional_domain_name
  }

  default_cache_behavior {
    allowed_methods        = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = local.s3_origin_id
    cache_policy_id        = aws_cloudfront_cache_policy.content.id
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  tags = {
    Service     = local.app_name
    Environment = var.env
  }
}
