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

resource "aws_ecr_repository" "twitter-followee-list" {
  name = "twitter-followee-list"

  image_tag_mutability = "MUTABLE"

  encryption_configuration {
    encryption_type = "AES256"
  }

  image_scanning_configuration {
    scan_on_push = false
  }

  tags = {
    Name = "twitter-followee-list"
    cost = var.cost_tag
  }
}

resource "aws_ecr_lifecycle_policy" "twitter-followee-list-lifecycle-policy" {
  repository = aws_ecr_repository.twitter-followee-list.name
  policy     = file("${path.module}/lifecycle_policy.json")
}

resource "aws_ecr_repository" "twitter-merge-lists" {
  name = "twitter-merge-lists"

  image_tag_mutability = "MUTABLE"

  encryption_configuration {
    encryption_type = "AES256"
  }

  image_scanning_configuration {
    scan_on_push = false
  }

  tags = {
    Name = "twitter-merge-lists"
    cost = var.cost_tag
  }
}

resource "aws_ecr_lifecycle_policy" "twitter-merge-lists-lifecycle-policy" {
  repository = aws_ecr_repository.twitter-merge-lists.name
  policy     = file("${path.module}/lifecycle_policy.json")
}

resource "aws_ecr_repository" "tonarinoyj-update-checker" {
  name = "tonarinoyj-update-checker"

  image_tag_mutability = "MUTABLE"

  encryption_configuration {
    encryption_type = "AES256"
  }

  image_scanning_configuration {
    scan_on_push = false
  }

  tags = {
    Name = "tonarinoyj-update-checker"
    cost = var.cost_tag
  }
}

resource "aws_ecr_lifecycle_policy" "tonarinoyj-update-checker-lifecycle-policy" {
  repository = aws_ecr_repository.tonarinoyj-update-checker.name
  policy     = file("${path.module}/lifecycle_policy.json")
}

resource "aws_ecr_repository" "gokabot-random-message-caller" {
  name = "gokabot-random-message-caller"

  image_tag_mutability = "MUTABLE"

  encryption_configuration {
    encryption_type = "AES256"
  }

  image_scanning_configuration {
    scan_on_push = false
  }

  tags = {
    Name = "gokabot-random-message-caller"
    cost = var.cost_tag
  }
}

resource "aws_ecr_lifecycle_policy" "gokabot-random-message-caller-lifecycle-policy" {
  repository = aws_ecr_repository.gokabot-random-message-caller.name
  policy     = file("${path.module}/lifecycle_policy.json")
}
