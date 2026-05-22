output "grafana_url" {
  value = "https://grafana.${var.environment}.cislunar.local"
}

output "prometheus_endpoint" {
  value = "https://prometheus.${var.environment}.cislunar.local"
}
