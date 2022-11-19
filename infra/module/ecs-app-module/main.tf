terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region = var.region
}

locals {
  full_name = "${var.service_name}-${var.env}"
}


module "iam" {
  source = "../ecs-iam-module"

  region       = var.region
  env          = var.env
  service_name = "code-service"
}


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

resource "aws_ecs_task_definition" "service" {
  family                   = "${local.full_name}-task-definition"
  requires_compatibilities = ["FARGATE"]
  cpu                      = var.cpu
  memory                   = var.memory
  network_mode             = "awsvpc"
  execution_role_arn       = module.iam.execution_role_arn

  runtime_platform {
    operating_system_family = "LINUX"
    cpu_architecture        = "ARM64"
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

resource "aws_appautoscaling_target" "service" {
  max_capacity       = 5
  min_capacity       = 1
  resource_id        = "service/${aws_ecs_cluster.service.name}/${aws_ecs_service.service.name}"
  scalable_dimension = "ecs:service:DesiredCount"
  service_namespace  = "ecs"
}


resource "aws_appautoscaling_policy" "memory" {
  name               = "${local.full_name}-memory-autoscaling"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.service.resource_id
  scalable_dimension = aws_appautoscaling_target.service.scalable_dimension
  service_namespace  = aws_appautoscaling_target.service.service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ECSServiceAverageMemoryUtilization"
    }

    target_value = 80
  }
}

resource "aws_appautoscaling_policy" "cpu" {
  name               = "${local.full_name}-cpu-autoscaling"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.service.resource_id
  scalable_dimension = aws_appautoscaling_target.service.scalable_dimension
  service_namespace  = aws_appautoscaling_target.service.service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ECSServiceAverageCPUUtilization"
    }

    target_value = 80
  }
}
