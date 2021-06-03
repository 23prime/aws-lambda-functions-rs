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
  policy = data.aws_iam_policy_document.sns-topic-policy.json
}

data "aws_iam_policy_document" "sns-topic-policy" {
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
      aws_sns_topic.notification-by-gokabot-topic.arn,
    ]

    sid = "__default_statement_ID"
  }
}

resource "aws_sns_topic_subscription" "notification-by-gokabot-topic-subscription" {
  topic_arn = aws_sns_topic.notification-by-gokabot-topic.arn
  protocol = "lambda"
  endpoint = var.lambda_function.arn
}
