output "notification-by-gokabot-repo" {
  value = aws_ecr_repository.notification-by-gokabot
}

output "notification-to-msteams-repo" {
  value = aws_ecr_repository.notification-to-msteams
}
