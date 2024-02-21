variable "environment" {}
variable "service" {}
variable "vpc_id" {}

variable "subnet_ids" {
  type = object({
    public  = list(string)
    private = list(string)
  })
}
