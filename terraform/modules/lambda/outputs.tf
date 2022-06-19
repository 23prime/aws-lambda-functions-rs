output "notification-by-gokabot" {
  value = aws_lambda_function.notification-by-gokabot
}

output "notification-to-msteams" {
  value = aws_lambda_function.notification-to-msteams
}

output "twitter-followee-list" {
  value = aws_lambda_function.twitter-followee-list
}

output "twitter-merge-vtubers-lists" {
  value = aws_lambda_function.twitter-merge-vtubers-lists
}

output "one-punch-man-update-checker" {
  value = aws_lambda_function.one-punch-man-update-checker
}

output "gokabot-random-message-caller" {
  value = aws_lambda_function.gokabot-random-message-caller
}
