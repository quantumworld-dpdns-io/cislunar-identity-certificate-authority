variable "environment" {
  type = string
}

variable "prometheus_retention_days" {
  type    = number
  default = 30
}
