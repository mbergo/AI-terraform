# Create an EC2 instance
resource "aws_instance" "ai_instance" {
  ami           = "ami-01234567890abcdef"
  instance_type = "g3.4xlarge"
  vpc_id        = aws_vpc.ai_vpc.id

  # Add other required configuration, such as security groups, key pairs, etc.
}

# Create an Elastic Inference Accelerator
resource "aws_eia" "ai_eia" {
  type            = "ml.eia1.medium"
  instance_type   = aws_instance.ai_instance.instance_type
  instance_id     = aws_instance.ai_instance.id
  volume_size     = 100
  vpc_id          = aws_vpc.ai_vpc.id
}
