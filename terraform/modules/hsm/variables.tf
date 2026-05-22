variable "enable_pqc" {
  type    = bool
  default = false
}

variable "environment" {
  type = string
}

variable "subnet_ids" {
  type    = list(string)
  default = []
}
