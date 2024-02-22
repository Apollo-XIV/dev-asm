#!/usr/bin/env bash
sudo yum update -y
sudo yum install -y docker python3
sudo service docker start
sudo usermod -aG docker ec2-user
