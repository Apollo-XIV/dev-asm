resource "aws_instance" "cluster_mgr" {
  ami           = data.aws_ami.aws_linux.id
  instance_type = "t3.micro"

  subnet_id       = var.subnet_ids.public[0]
  security_groups = [aws_security_group.node.id]

  key_name = aws_key_pair.node_key_2.key_name

  # user_data = "sudo  upgrade"

  connection {
    type        = "ssh"
    user        = "ec2-user"
    private_key = file("${path.module}/node_key_2")
    host        = self.public_ip
  }

  provisioner "remote-exec" {
    script = "${path.module}/install-docker.sh"
  }

  provisioner "remote-exec" {
    inline = [
      "docker swarm init --advertise-addr $(hostname -I | awk '{print $1}') &> /dev/null"
    ]
  }

  tags = {
    Name = "Cluster Manager"
  }
}

resource "aws_instance" "cluster_node" {
  count         = 2
  ami           = data.aws_ami.aws_linux.id
  instance_type = "t3.micro"

  subnet_id       = var.subnet_ids.public[0]
  security_groups = [aws_security_group.node.id]

  key_name = aws_key_pair.node_key_2.key_name

  connection {
    type        = "ssh"
    user        = "ec2-user"
    private_key = file("${path.module}/node_key_2")
    host        = self.public_ip
  }

  provisioner "remote-exec" {
    script = "${path.module}/install-docker.sh"
  }

  provisioner "remote-exec" {
    inline = [
      "docker swarm join --token ${data.external.swarm_join_token.result.worker} ${aws_instance.cluster_mgr.private_ip}:2377"
    ]
  }

  tags = {
    Name = "Cluster Manager"
  }
}

data "external" "swarm_join_token" {
  program     = ["sh", "get-join-tokens.sh"]
  working_dir = path.module
  query = {
    host = "${aws_instance.cluster_mgr.public_ip}"
  }
}

data "aws_ami" "aws_linux" {

  filter {
    name   = "name"
    values = ["al2023-ami-2023.3.20240219.0-kernel-6.1-x86_64"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  owners = ["137112412989"] # Aws
}

resource "aws_key_pair" "node_key_2" {
  key_name   = "node_key_2"
  public_key = file("${path.module}/node_key_2.pub")
}
