resource "aws_ecr_repository" "notification-by-gokabot" {
  name = "notification-by-gokabot"

  image_tag_mutability = "MUTABLE"

  encryption_configuration {
    encryption_type = "AES256"
  }

  image_scanning_configuration {
    scan_on_push = false
  }

  tags = {
    Name = "notification-by-gokabot"
    cost = var.cost_tag
  }
}

resource "aws_ecr_lifecycle_policy" "notification-by-gokabot-lifecycle-policy" {
  repository = aws_ecr_repository.notification-by-gokabot.name
  policy     = file("${path.module}/lifecycle_policy.json")
}

resource "aws_ecr_repository" "notification-to-msteams" {
  name = "notification-to-msteams"

  image_tag_mutability = "MUTABLE"

  encryption_configuration {
    encryption_type = "AES256"
  }

  image_scanning_configuration {
    scan_on_push = false
  }

  tags = {
    Name = "notification-to-msteams"
    cost = var.cost_tag
  }
}

resource "aws_ecr_lifecycle_policy" "notification-to-msteams-lifecycle-policy" {
  repository = aws_ecr_repository.notification-to-msteams.name
  policy     = file("${path.module}/lifecycle_policy.json")
}
