output "twitter-followee-list-schedule" {
  value = aws_cloudwatch_event_rule.twitter-followee-list-schedule
}

output "twitter-merge-vtubers-lists-schedule" {
  value = aws_cloudwatch_event_rule.twitter-merge-vtubers-lists-schedule
}

output "one-punch-man-update-checker-schedule" {
  value = aws_cloudwatch_event_rule.one-punch-man-update-checker-schedule
}
