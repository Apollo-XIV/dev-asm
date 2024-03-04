
resource "aws_db_instance" "default" {
  allocated_storage           = 5
  db_name                     = "forum"
  engine                      = "postgres"
  instance_class              = "db.t3.micro"
  username                    = "backend"
  manage_master_user_password = true
  skip_final_snapshot         = true
}

output "db_cx_string" {
  value = aws_db_instance.default.endpoint
}

resource "local_file" "cx_string" {
  filename = "playbooks/cx_string"
  content  = aws_db_instance.default.endpoint
}
