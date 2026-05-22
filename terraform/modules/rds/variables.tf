variable "db_name" {
  type    = string
  default = "cislunar_ca"
}

variable "engine_version" {
  type    = string
  default = "16.3"
}

variable "environment" {
  type    = string
}

variable "subnet_ids" {
  type    = list(string)
  default = []
}
