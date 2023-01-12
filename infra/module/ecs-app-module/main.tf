terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

locals {
  full_name = "${var.service_name}-${var.env}"
}

resource "aws_ecs_task_definition" "service" {
  family                   = "${local.full_name}-task-definition"
  requires_compatibilities = ["FARGATE"]
  cpu                      = var.cpu
  memory                   = var.memory
  network_mode             = "awsvpc"
  execution_role_arn       = aws_iam_role.ecs_task_execution_role.arn

  runtime_platform {
    operating_system_family = "LINUX"
    cpu_architecture        = "X86_64"
  }

  container_definitions = jsonencode([
    {
      name        = local.full_name
      image       = "${var.repository_url}:${var.repository_tag}"
      cpu         = var.cpu
      memory      = var.memory
      networkMode = "awsvpc"
      environment = [
        {
          name  = "PORT"
          value = "8080"
        }
      ]
      logConfiguration = {
        logDriver = "awslogs"
        options = {
          awslogs-group         = aws_cloudwatch_log_group.service.id
          awslogs-region        = var.region
          awslogs-stream-prefix = local.full_name
        }
      }
      portMappings = [
        {
          containerPort = 8080
          hostPort      = 8080
        }
      ]

    }
  ])

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}

resource "aws_security_group" "service" {
  name   = "${local.full_name}-task-security-group"
  vpc_id = var.vpc_id

  ingress {
    protocol        = "tcp"
    from_port       = 8080
    to_port         = 8080
    security_groups = [aws_security_group.lb.id]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}

resource "aws_ecs_cluster" "service" {
  name = "${local.full_name}-cluster"

  setting {
    name  = "containerInsights"
    value = "enabled"
  }

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}

resource "aws_ecs_service" "service" {
  name            = "${local.full_name}-service"
  cluster         = aws_ecs_cluster.service.id
  task_definition = aws_ecs_task_definition.service.arn
  desired_count   = var.desired_count
  launch_type     = "FARGATE"

  network_configuration {
    security_groups  = [aws_security_group.service.id]
    subnets          = var.public_subnet_ids
    assign_public_ip = true
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.service.id
    container_name   = local.full_name
    container_port   = 8080
  }

  depends_on = [aws_lb_listener.service]

  tags = {
    Service     = var.service_name
    Environment = var.env
  }
}
