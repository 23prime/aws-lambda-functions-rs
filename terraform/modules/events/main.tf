resource "aws_cloudwatch_event_rule" "twitter-followee-list-schedule" {
  name        = "twitter-followee-list-schedule"
  description = "Scheduled event for Lambda function: twitter-followee-list"

  # cron: https://docs.aws.amazon.com/ja_jp/AmazonCloudWatch/latest/events/ScheduledEvents.html
  schedule_expression = "cron(0 * * * ? *)"

  tags = {
    Name = "twitter-followee-list-schedule"
    cost = var.cost_tag
  }
}

resource "aws_cloudwatch_event_target" "twitter-followee-list-schedule" {
  rule = aws_cloudwatch_event_rule.twitter-followee-list-schedule.name
  arn  = var.lambda_function.twitter-followee-list.arn
}
