resource "aws_security_group" "db" {
  name        = "forum-db-security-group"
  description = "security group for cluster nodes"
  vpc_id      = var.vpc_id

  ingress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
    self        = true
  }

  egress {
    from_port = 0
    to_port   = 0
    protocol  = "-1"
    self      = true
  }

  tags = {
    Name = "${var.service}-${var.environment}-db-grp"
  }
}
