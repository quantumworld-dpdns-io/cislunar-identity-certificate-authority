resource "aws_s3_bucket" "prometheus_data" {
  bucket = "${var.environment}-cislunar-ca-prometheus"
}

resource "aws_s3_bucket" "grafana_data" {
  bucket = "${var.environment}-cislunar-ca-grafana"
}

resource "aws_iam_role" "prometheus" {
  name = "${var.environment}-cislunar-ca-prometheus"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        Service = "ec2.amazonaws.com"
      }
      Action = "sts:AssumeRole"
    }]
  })
}

resource "aws_iam_role" "grafana" {
  name = "${var.environment}-cislunar-ca-grafana"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        Service = "ec2.amazonaws.com"
      }
      Action = "sts:AssumeRole"
    }]
  })
}
