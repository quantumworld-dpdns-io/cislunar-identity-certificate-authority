output "cluster_endpoint" {
  description = "EKS cluster endpoint"
  value       = module.eks.cluster_endpoint
}

output "rds_endpoint" {
  description = "RDS instance endpoint"
  value       = module.rds.endpoint
}

output "monitoring_grafana_url" {
  description = "Grafana dashboard URL"
  value       = module.monitoring.grafana_url
}
