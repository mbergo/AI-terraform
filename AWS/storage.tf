# Create an S3 bucket
resource "aws_s3_bucket" "ai_data" {
  bucket = "ai-data"
}
