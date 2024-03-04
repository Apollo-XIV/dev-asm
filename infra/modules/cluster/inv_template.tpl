---
managers:
  hosts:
  %{~ for ec2_name, address in managers ~}
    ${ec2_name}:
      ansible_host: ${address}
  %{~ endfor ~}
  vars:
    ansible_user: ec2-user
    ansible_ssh_private_key_file: ../modules/cluster/node_key
workers:
  hosts:
  %{~ for ec2_name, address in workers ~}
    ${ec2_name}:
      ansible_host: ${address}
  %{~ endfor ~}
  vars:
    ansible_user: ec2-user
    ansible_ssh_private_key_file: ../modules/cluster/node_key
