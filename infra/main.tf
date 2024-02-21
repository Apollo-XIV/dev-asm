provider "aws" {
  region = "eu-west-1"
}

module "network" {
  source      = "./modules/network"
  cidr        = var.cidr
  service     = var.service
  environment = var.environment
}

module "cluster" {
  source      = "./modules/cluster"
  service     = var.service
  environment = var.environment
  subnet_ids  = module.network.subnet_ids
  vpc_id      = module.network.vpc_id
}
