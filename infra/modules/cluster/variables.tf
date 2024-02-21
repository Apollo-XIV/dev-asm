variable "environment" {}
variable "service" {}
variable "vpc_id" {}

variable "subnet_ids" {
  type = object({
    public  = list(string)
    private = list(string)
  })
}


resource "local_file" "ansible_inventory" {
  filename = "inventory.yml"
  content = yamlencode({
    managers = {
      hosts = [
        aws_instance.cluster_mgr.public_ip
      ]
    }
    workers = {
      hosts = aws_instance.cluster_node[*].public_ip
    }
  })
}
