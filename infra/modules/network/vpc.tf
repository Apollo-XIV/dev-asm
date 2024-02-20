resource "aws_vpc" "main" {
  cidr_block = var.cidr

  tags = {
    Name = "${var.service}-${var.environment}-vpc"
  }
}

resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.main.id
  tags = {
    Name = "${var.service}-${var.environment}-igw"
  }
}

resource "aws_eip" "nat_gw_eip" {
  domain = "vpc"
  tags = {
    Name = "${var.service}-${var.environment}-eip"
  }

  # lifecycle {
  #   prevent_destroy = true
  # }
}

resource "aws_nat_gateway" "nat_gw" {
  allocation_id = aws_eip.nat_gw_eip.id
  subnet_id     = aws_subnet.public[0].id
}

data "aws_availability_zones" "available" {}

locals {
  availability_zones = slice(data.aws_availability_zones.available.names, 0, var.availability_zones)
}

resource "aws_subnet" "public" {
  vpc_id            = aws_vpc.main.id
  count             = var.availability_zones
  cidr_block        = cidrsubnet(var.cidr, 4, count.index)
  availability_zone = local.availability_zones[count.index]
}

resource "aws_subnet" "private" {
  vpc_id            = aws_vpc.main.id
  count             = var.availability_zones
  cidr_block        = cidrsubnet(var.cidr, 4, (8 + count.index))
  availability_zone = local.availability_zones[count.index]
}
