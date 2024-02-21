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

output "subnet_ids" {
  value = {
    public  = aws_subnet.public[*].id
    private = aws_subnet.private[*].id
  }
}
