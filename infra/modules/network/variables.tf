variable "service" {}
variable "environment" {}
variable "cidr" {}

variable "availability_zones" {
  type    = number
  default = 1
}
