# See cron settings: https://docs.aws.amazon.com/ja_jp/AmazonCloudWatch/latest/events/ScheduledEvents.html
resource "aws_cloudwatch_event_rule" "twitter-followee-list-schedule" {
  name        = "twitter-followee-list-schedule"
  description = "Scheduled event for Lambda function: twitter-followee-list"

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

resource "aws_cloudwatch_event_rule" "twitter-merge-vtubers-lists-schedule" {
  name        = "twitter-merge-vtubers-lists-schedule"
  description = "Scheduled event for Lambda function: twitter-merge-vtubers-lists"

  schedule_expression = "cron(0 * * * ? *)"

  tags = {
    Name = "twitter-merge-vtubers-lists-schedule"
    cost = var.cost_tag
  }
}

resource "aws_cloudwatch_event_target" "twitter-merge-vtubers-lists-schedule" {
  rule = aws_cloudwatch_event_rule.twitter-merge-vtubers-lists-schedule.name
  arn  = var.lambda_function.twitter-merge-vtubers-lists.arn
}

resource "aws_cloudwatch_event_rule" "one-punch-man-update-checker-schedule" {
  name        = "one-punch-man-update-checker-schedule"
  description = "Scheduled event for Lambda function: one-punch-man-update-checker"

  schedule_expression = "cron(0 * * * ? *)"

  tags = {
    Name = "one-punch-man-update-checker-schedule"
    cost = var.cost_tag
  }
}

resource "aws_cloudwatch_event_target" "one-punch-man-update-checker-schedule" {
  rule = aws_cloudwatch_event_rule.one-punch-man-update-checker-schedule.name
  arn  = var.lambda_function.one-punch-man-update-checker.arn
}
