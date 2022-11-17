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
  az_count = length(data.aws_availability_zones.available_zones.names)
}

data "aws_availability_zones" "available_zones" {
  state = "available"
}

resource "aws_vpc" "default" {
  cidr_block = var.cidr

  tags = {
    App = var.app_name
    Env = var.env
  }
}


resource "aws_internet_gateway" "default" {
  vpc_id = aws_vpc.default.id

  tags = {
    App = var.app_name
    Env = var.env
  }
}

resource "aws_subnet" "public" {
  count                   = local.az_count
  cidr_block              = cidrsubnet(aws_vpc.default.cidr_block, 8, local.az_count + count.index)
  availability_zone       = data.aws_availability_zones.available_zones.names[count.index]
  vpc_id                  = aws_vpc.default.id
  map_public_ip_on_launch = true

  tags = {
    Type = "Public"
    App  = var.app_name
    Env  = var.env
  }
}

resource "aws_subnet" "private" {
  count             = length(data.aws_availability_zones.available_zones.names)
  cidr_block        = cidrsubnet(aws_vpc.default.cidr_block, 8, count.index)
  availability_zone = data.aws_availability_zones.available_zones.names[count.index]
  vpc_id            = aws_vpc.default.id

  tags = {
    Type = "Private"
    App  = var.app_name
    Env  = var.env
  }
}

resource "aws_route_table" "public" {
  vpc_id = aws_vpc.default.id
  count  = local.az_count

  tags = {
    Type = "Public"
    App  = var.app_name
    Env  = var.env
  }
}

resource "aws_route" "public" {
  count                  = local.az_count
  route_table_id         = aws_route_table.public[count.index].id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.default.id
}

resource "aws_route_table_association" "public-route-association" {
  count          = local.az_count
  subnet_id      = aws_subnet.public[count.index].id
  route_table_id = aws_route_table.public[count.index].id
}

resource "aws_route_table" "private" {
  vpc_id = aws_vpc.default.id
  count  = local.az_count

  tags = {
    Type = "Private"
    App  = var.app_name
    Env  = var.env
  }
}

resource "aws_route" "private" {
  count                  = local.az_count
  route_table_id         = aws_route_table.private[count.index].id
  destination_cidr_block = "0.0.0.0/0"
  nat_gateway_id         = aws_nat_gateway.default[count.index].id
}

resource "aws_route_table_association" "private_route_association" {
  count          = local.az_count
  subnet_id      = aws_subnet.private[count.index].id
  route_table_id = aws_route_table.private[count.index].id
}

resource "aws_eip" "gateway" {
  count      = local.az_count
  vpc        = true
  depends_on = [aws_internet_gateway.default]
}

resource "aws_nat_gateway" "default" {
  count         = local.az_count
  subnet_id     = aws_subnet.public[count.index].id
  allocation_id = aws_eip.gateway[count.index].id
}
