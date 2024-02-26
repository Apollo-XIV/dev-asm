
resource "aws_db_instance" "default" {
  allocated_storage           = 5
  db_name                     = "forum"
  engine                      = "postgresql"
  instance_class              = "db.t3.micro"
  username                    = "backend"
  manage_master_user_password = true
}
