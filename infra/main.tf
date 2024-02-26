
# module "ansible" {
#   source    = "./playbooks"
#   hostnames = module.cluster.hostnames
# }

module "network" {
  source      = "./modules/network"
  cidr        = var.cidr
  service     = var.service
  environment = var.environment
  availability_zones = 2
}

module "cluster" {
  source      = "./modules/cluster"
  service     = var.service
  environment = var.environment
  subnet_ids  = module.network.subnet_ids
  vpc_id      = module.network.vpc_id
}
