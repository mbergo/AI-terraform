# Create a Secrets Manager
resource "aws_secretsmanager_secret" "ai_secret" {
  name = "ai-secret"
}
