
resource "aws_db_instance" "default" {
  allocated_storage    = 5
  db_name              = "forum"
  db_subnet_group_name = aws_db_subnet_group.forum.name
  publicly_accessible  = true
  engine               = "postgres"
  instance_class       = "db.t3.micro"
  username             = "backend"
  password             = random_password.db_key.result
  skip_final_snapshot  = true
  storage_encrypted    = true
}

resource "aws_db_subnet_group" "forum" {
  name       = "forum_snets"
  subnet_ids = var.subnet_ids.public[*]
}

resource "random_password" "db_key" {
  length           = 16
  special          = true
  override_special = "!#$%&*()-_=+[]{}<>?"
}

variable "subnet_ids" {
  type = object({
    public  = list(string)
    private = list(string)
  })
}

output "db_cx_string" {
  value = aws_db_instance.default.endpoint
}

resource "local_sensitive_file" "cx_string" {
  filename = "playbooks/cx_string"
  content  = "postgres://backend:${random_password.db_key.result}@${aws_db_instance.default.endpoint}/${aws_db_instance.default.db_name}"
}
