resource "aws_security_group" "lb" {
  name   = "${var.service_name}-${var.env}-lb-security-group"
  vpc_id = var.vpc_id

  ingress {
    protocol    = "tcp"
    from_port   = 80
    to_port     = 80
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}

resource "aws_lb" "service" {
  name            = "${local.full_name}-lb"
  subnets         = var.public_subnet_ids
  security_groups = [aws_security_group.lb.id]

  access_logs {
    bucket  = aws_s3_bucket.lb_logs.bucket
    prefix  = "${local.full_name}-lb"
    enabled = true
  }

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}

resource "aws_lb_target_group" "service" {
  name        = "${local.full_name}-target-group"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    healthy_threshold   = "3"
    interval            = "300"
    protocol            = "HTTP"
    matcher             = "200"
    timeout             = "3"
    unhealthy_threshold = "2"
    path                = "/api/v1/dart/version"
  }

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}

resource "aws_lb_listener" "service" {
  load_balancer_arn = aws_lb.service.id
  port              = "80"
  protocol          = "HTTP"

  default_action {
    target_group_arn = aws_lb_target_group.service.id
    type             = "forward"
  }

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}
