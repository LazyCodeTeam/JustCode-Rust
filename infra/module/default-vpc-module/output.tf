output "vpc_id" {
  value = data.aws_vpc.default.id
}

output "private_subnet_ids" {
  value = []
}

output "public_subnet_ids" {
  value = data.aws_subnets.default.ids[*]
}
