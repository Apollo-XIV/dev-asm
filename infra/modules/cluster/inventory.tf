resource "local_file" "inventory" {
  filename = "playbooks/inventory.yml"
  content = templatefile("${path.module}/inv_template.tpl",
    {
      managers = tomap({
        for instance in [aws_instance.bootstrap] :
        instance.tags.Name => instance.public_dns
      }),
      workers = tomap({
        for instance in aws_instance.workers[*] :
        instance.tags.Name => instance.public_dns
      })
  })
}

# resource "ansible_group" "managers" {
#   name     = "managers"
#   children = [ansible_host.bootstrap.name]
# }

# resource "ansible_group" "workers" {
#   name     = "workers"
#   children = []
# }

# resource "ansible_host" "bootstrap" {
#   name   = var.hostnames.managers[0]
#   groups = ["managers"]
#   variables = {
#     ansible_user                 = "ec2-user",
#     ansible_ssh_private_key_file = "${path.module}../modules/cluster/node_key",
#     ansible_python_interpreter   = "/usr/bin/python3"
#   }
# }


# resource "ansible_playbook" "hello_world" {
#   playbook   = "${path.module}/hello_world.yml"
#   name       = 
#   groups     = [ansible_group.workers[*].name]
#   replayable = true
# }
