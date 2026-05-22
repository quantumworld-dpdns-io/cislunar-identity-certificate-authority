variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-west-2"
}

variable "cluster_name" {
  description = "EKS cluster name"
  type        = string
  default     = "cislunar-ca"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "production"
}

variable "eks_instance_types" {
  description = "EKS node instance types"
  type        = list(string)
  default     = ["m6i.large", "c6i.large"]
}

variable "eks_node_count" {
  description = "EKS node count"
  type        = number
  default     = 3
}

variable "rds_engine_version" {
  description = "PostgreSQL engine version"
  type        = string
  default     = "16.3"
}

variable "enable_pqc" {
  description = "Enable Post-Quantum Cryptography support"
  type        = bool
  default     = false
}

variable "prometheus_retention_days" {
  description = "Prometheus metrics retention days"
  type        = number
  default     = 30
}

variable "github_repo" {
  description = "GitHub repository for CI runner"
  type        = string
  default     = "quantumworld-dpdns-io/cislunar-identity-certificate-authority"
}
