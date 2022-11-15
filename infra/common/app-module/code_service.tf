locals {
  service_name = "CodeService"
}

resource "aws_security_group" "code_service_lb" {
  name   = "code-service-http-security-group"
  vpc_id = data.aws_vpc.default.id

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
    Environment = var.env
    Service     = local.service_name
  }
}

resource "aws_lb" "code_service" {
  name            = "code-service-lb"
  subnets         = data.aws_subnets.default.ids
  security_groups = [aws_security_group.code_service_lb.id]

  tags = {
    Environment = var.env
    Service     = local.service_name
  }
}

resource "aws_lb_target_group" "code_service" {
  name        = "code-service-target-group"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = data.aws_vpc.default.id
  target_type = "ip"

  health_check {
    enabled = false
  }

  tags = {
    Environment = var.env
    Service     = local.service_name
  }
}

resource "aws_lb_listener" "code_service" {
  load_balancer_arn = aws_lb.code_service.id
  port              = "80"
  protocol          = "HTTP"

  default_action {
    target_group_arn = aws_lb_target_group.code_service.id
    type             = "forward"
  }

  tags = {
    Environment = var.env
    Service     = local.service_name
  }
}

resource "aws_ecs_task_definition" "code_service" {
  family                   = "code-service-task-definition"
  requires_compatibilities = ["FARGATE"]
  container_definitions = jsonencode([
    {
      image       = "${aws_ecr_repository.code_service.repository_url}:${var.code_service_tag}"
      cpu         = 1024
      memory      = 2048
      name        = "code-service"
      networkMode = "awsvpc"
      portMappings = [
        {
          containerPort = 8080
          hostPort      = 8080
        }
      ]
    }
  ])

  tags = {
    Environment = var.env
    Service     = local.service_name
  }
}

resource "aws_security_group" "code_service_task" {
  name   = "example-task-security-group"
  vpc_id = data.aws_vpc.default.id

  ingress {
    protocol        = "tcp"
    from_port       = 8080
    to_port         = 8080
    security_groups = [aws_security_group.code_service_lb.id]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Environment = var.env
    Service     = local.service_name
  }
}

output "code_service_load_balancer_ip" {
  value = aws_lb.code_service.dns_name
}
