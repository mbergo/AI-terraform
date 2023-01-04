# Create a CloudWatch Alarm
resource "aws_cloudwatch_metric_alarm" "ai_alarm" {
  alarm_name          = "ai-alarm"
  comparison_operator = "GreaterThanOrEqualToThreshold"
  evaluation_periods  = "1"
  metric_name         = "CPUUtilization"
  namespace           = "AWS/EC2"
  period              = "60"
  statistic           = "Average"
  threshold           = "70"
}
