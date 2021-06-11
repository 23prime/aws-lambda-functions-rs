resource "aws_sns_topic" "notification-by-gokabot-topic" {
  name         = "notification-by-gokabot"
  display_name = "notification-by-gokabot"

  tags = {
    Name = "notification-by-gokabot"
    cost = var.cost_tag
  }
}

resource "aws_sns_topic_policy" "notification-by-gokabot-topic-policy" {
  arn    = aws_sns_topic.notification-by-gokabot-topic.arn
  policy = data.aws_iam_policy_document.notification-by-gokabot-topic-policy-document.json
}

data "aws_iam_policy_document" "notification-by-gokabot-topic-policy-document" {
  policy_id = "__default_policy_ID"

  statement {
    actions = [
      "SNS:GetTopicAttributes",
      "SNS:SetTopicAttributes",
      "SNS:AddPermission",
      "SNS:RemovePermission",
      "SNS:DeleteTopic",
      "SNS:Subscribe",
      "SNS:ListSubscriptionsByTopic",
      "SNS:Publish",
      "SNS:Receive",
    ]

    condition {
      test     = "StringEquals"
      variable = "AWS:SourceOwner"
      values = [
        var.aws_account_id
      ]
    }

    effect = "Allow"

    principals {
      type        = "AWS"
      identifiers = ["*"]
    }

    resources = [
      aws_sns_topic.notification-by-gokabot-topic.arn
    ]

    sid = "__default_statement_ID"
  }
}

resource "aws_sns_topic_subscription" "notification-by-gokabot-topic-subscription" {
  topic_arn = aws_sns_topic.notification-by-gokabot-topic.arn
  protocol  = "lambda"
  endpoint  = var.lambda_function.notification-by-gokabot.arn
}

resource "aws_sns_topic" "notification-to-msteams-topic" {
  name         = "notification-to-msteams"
  display_name = "notification-to-msteams"

  tags = {
    Name = "notification-to-msteams"
    cost = var.cost_tag
  }
}

resource "aws_sns_topic_policy" "notification-to-msteams-topic-policy" {
  arn    = aws_sns_topic.notification-to-msteams-topic.arn
  policy = data.aws_iam_policy_document.notification-to-msteams-topic-policy-document.json
}

data "aws_iam_policy_document" "notification-to-msteams-topic-policy-document" {
  policy_id = "__default_policy_ID"

  statement {
    actions = [
      "SNS:GetTopicAttributes",
      "SNS:SetTopicAttributes",
      "SNS:AddPermission",
      "SNS:RemovePermission",
      "SNS:DeleteTopic",
      "SNS:Subscribe",
      "SNS:ListSubscriptionsByTopic",
      "SNS:Publish",
      "SNS:Receive",
    ]

    condition {
      test     = "StringEquals"
      variable = "AWS:SourceOwner"
      values = [
        var.aws_account_id
      ]
    }

    effect = "Allow"

    principals {
      type        = "AWS"
      identifiers = ["*"]
    }

    resources = [
      aws_sns_topic.notification-to-msteams-topic.arn
    ]

    sid = "__default_statement_ID"
  }
}

resource "aws_sns_topic_subscription" "notification-to-msteams-topic-subscription" {
  topic_arn = aws_sns_topic.notification-to-msteams-topic.arn
  protocol  = "lambda"
  endpoint  = var.lambda_function.notification-to-msteams.arn
}
