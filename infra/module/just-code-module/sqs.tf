resource "aws_sqs_queue" "tasks_migration" {
  name                       = "tasks-migration-${var.env}-queue"
  visibility_timeout_seconds = 60
  delay_seconds              = 60
  receive_wait_time_seconds  = 20
  sqs_managed_sse_enabled    = true

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
  name = "tasks-migration-${var.env}-deadletter-queue"

  tags = {
    Service     = local.app_name
    Environment = var.env
  }
}
