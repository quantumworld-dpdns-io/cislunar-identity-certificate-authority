resource "aws_cloudhsm_v2_cluster" "main" {
  count = var.enable_pqc ? 1 : 0
  hsm_type = "hsm1.medium"
  subnet_ids = var.subnet_ids
}

resource "aws_kms_key" "ca_master" {
  description             = "Cislunar CA Master Key"
  deletion_window_in_days = 30
  enable_key_rotation     = true

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        AWS = "arn:aws:iam::${data.aws_caller_identity.current.account_id}:root"
      }
      Action = "kms:*"
      Resource = "*"
    }]
  })
}

resource "aws_kms_alias" "ca_master" {
  name          = "alias/cislunar-ca-master"
  target_key_id = aws_kms_key.ca_master.key_id
}
