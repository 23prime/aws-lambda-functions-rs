output "notification-by-gokabot-repo" {
  value = aws_ecr_repository.notification-by-gokabot
}

output "notification-to-msteams-repo" {
  value = aws_ecr_repository.notification-to-msteams
}

output "twitter-followee-list-repo" {
  value = aws_ecr_repository.twitter-followee-list
}

output "twitter-merge-lists-repo" {
  value = aws_ecr_repository.twitter-merge-lists
}
