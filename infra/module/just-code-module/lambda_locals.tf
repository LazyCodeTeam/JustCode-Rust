locals {
  env_vars = {
    TASK_MIGRATION_SQS_QUEUE = aws_sqs_queue.tasks_migration.url

    DYNAMODB_TABLE         = aws_dynamodb_table.main.name
    CONTENT_DYNAMODB_TABLE = aws_dynamodb_table.content.name

    S3_BUCKET       = aws_s3_bucket.content.id
    BUCKET_BASE_URL = "https://${aws_cloudfront_distribution.content.domain_name}"
  }
}
