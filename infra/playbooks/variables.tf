terraform {

  required_providers {
    ansible = {
      version = "~> 1.1.0"
      source  = "ansible/ansible"
    }
  }
}

variable "hostnames" {
  type = object({
    managers = list(string)
    workers  = list(string)
  })
}
