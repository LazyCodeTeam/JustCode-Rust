resource "aws_iam_role" "lambda_exec" {
  name = "${var.name}-${var.env}-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
        Effect = "Allow"
        Sid    = ""
      },
    ]
  })
}

resource "aws_iam_role_policy" "custom_lambda_policy" {
  for_each = var.policies_jsons

  name   = "${var.name}-${var.env}-policy-${each.key}"
  role   = aws_iam_role.lambda_exec.name
  policy = each.value
}

resource "aws_iam_role_policy_attachment" "basic_lambda_policy" {
  role       = aws_iam_role.lambda_exec.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "lambda_policy" {
  count = length(var.policies)

  role       = aws_iam_role.lambda_exec.name
  policy_arn = var.policies[count.index]
}
