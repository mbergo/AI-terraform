# Create an Internet Gateway
resource "aws_internet_gateway" "ai_igw" {
  vpc_id = aws_vpc.ai_vpc.id
}
