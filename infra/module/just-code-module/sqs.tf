resource "aws_sqs_queue" "tasks_migration" {
  name                    = "tasks-migration-${var.env}-queue"
  delay_seconds           = 0
  sqs_managed_sse_enabled = true
  redrive_policy = jsonencode({
    deadLetterTargetArn = aws_sqs_queue.tasks_migration_deadletter.arn
    maxReceiveCount     = 5
  })

  tags = {
    Service     = local.app_name
    Environment = var.env
  }
}

resource "aws_sqs_queue" "tasks_migration_deadletter" {
  name                      = "tasks-migration-${var.env}-deadletter-queue"
  delay_seconds             = 0
  message_retention_seconds = 345600
  sqs_managed_sse_enabled   = true

  tags = {
    Service     = local.app_name
    Environment = var.env
  }
}
