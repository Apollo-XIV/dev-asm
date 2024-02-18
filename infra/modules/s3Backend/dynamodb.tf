
#  a new AWS DynamoDB table resource 
resource "aws_dynamodb_table" "state_locking_table" {
  name = "${var.service}-${var.environment}-locktable"

  hash_key       = "LockID"
  read_capacity  = 20
  write_capacity = 20

  # an attribute for the DynamoDB table
  attribute {
    name = "LockID"
    type = "S" # Attribute type (String)
  }

  server_side_encryption {
    enabled     = true
    kms_key_arn = aws_kms_key.cmk_dynamo.arn
  }

  point_in_time_recovery {
    enabled = true
  }

  # tags for the DynamoDB table for better organization
  tags = {
    Name = "${var.service}-${var.environment}-locktable"
  }

  lifecycle {
    prevent_destroy = true
  }
}

resource "aws_kms_key" "cmk_dynamo" {
  description         = "Customer Managed Key for DynamoDB encryption"
  key_usage           = "ENCRYPT_DECRYPT"
  enable_key_rotation = true
  multi_region        = false
}