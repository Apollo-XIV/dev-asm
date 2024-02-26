#!/bin/bash
sudo yum update -y
sudo yum install -y docker python3 pip
sudo yum remove -y python-requests aws-cli
sudo service docker start	
sudo usermod -aG docker ec2-user
pip uninstall awscli
pip install docker
