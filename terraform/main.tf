terraform {
  required_version = ">= 1.5"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.25"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.12"
    }
  }
}

provider "aws" {
  region = var.aws_region
}

module "eks" {
  source = "./modules/eks"
  cluster_name    = var.cluster_name
  environment     = var.environment
  instance_types  = var.eks_instance_types
  node_count      = var.eks_node_count
}

module "rds" {
  source = "./modules/rds"
  db_name     = "cislunar_ca"
  environment = var.environment
  engine_version = var.rds_engine_version
}

module "hsm" {
  source = "./modules/hsm"
  environment = var.environment
  enable_pqc = var.enable_pqc
}

module "monitoring" {
  source = "./modules/monitoring"
  environment = var.environment
  prometheus_retention_days = var.prometheus_retention_days
}

module "ci_runner" {
  source = "./modules/ci-runner"
  environment = var.environment
  github_repo = var.github_repo
}
