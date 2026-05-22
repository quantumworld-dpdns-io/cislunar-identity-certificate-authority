resource "aws_db_instance" "main" {
  identifier        = "${var.environment}-cislunar-ca"
  engine            = "postgres"
  engine_version    = var.engine_version
  instance_class    = "db.r6g.large"
  allocated_storage = 100
  storage_type      = "gp3"
  storage_encrypted = true

  db_name  = var.db_name
  username = "cislunar_ca"
  password = random_password.db_password.result

  backup_retention_period = 30
  backup_window          = "03:00-04:00"
  maintenance_window     = "sun:04:00-sun:05:00"

  deletion_protection = true
  skip_final_snapshot = false

  vpc_security_group_ids = [aws_security_group.rds.id]
  db_subnet_group_name   = aws_db_subnet_group.main.name

  enabled_cloudwatch_logs_exports = ["postgresql", "upgrade"]
}

resource "random_password" "db_password" {
  length  = 32
  special = false
}

resource "aws_security_group" "rds" {
  name        = "${var.environment}-cislunar-ca-rds"
  description = "RDS security group"
}

resource "aws_db_subnet_group" "main" {
  name       = "${var.environment}-cislunar-ca"
  subnet_ids = var.subnet_ids
}
