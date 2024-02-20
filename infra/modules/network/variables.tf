variable "service" {}
variable "environment" {}
variable "cidr" {}

variable "availability_zones" {
  type    = number
  default = 1
}

output "vpc_id" {
  value = aws_vpc.main.id
}
