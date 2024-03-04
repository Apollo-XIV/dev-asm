variable "environment" {}
variable "service" {}
variable "vpc_id" {}

variable "subnet_ids" {
  type = object({
    public  = list(string)
    private = list(string)
  })
}


output "hostnames" {
  value = {
    managers = [aws_instance.bootstrap.public_dns]
    workers  = aws_instance.workers[*].private_dns
  }
}
# resource "local_file" "ansible_inventory" {
#   filename = "inventory.yml"
#   content = yamlencode({
#     managers = {
#       hosts = [
#         aws_instance.bootstrap.public_ip
#       ]
#     }
#     workers = {
#       hosts = aws_instance.[*].public_ip
#     }
#   })
# }
