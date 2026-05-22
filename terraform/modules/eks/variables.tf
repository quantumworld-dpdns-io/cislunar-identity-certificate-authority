variable "cluster_name" {
  type = string
}

variable "subnet_ids" {
  type    = list(string)
  default = []
}

variable "instance_types" {
  type    = list(string)
  default = ["m6i.large"]
}

variable "node_count" {
  type    = number
  default = 3
}

variable "environment" {
  type    = string
  default = "production"
}
