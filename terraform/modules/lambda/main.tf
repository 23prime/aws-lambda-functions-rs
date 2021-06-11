resource "aws_lambda_function" "notification-by-gokabot" {
  function_name = "notification-by-gokabot"

  role         = var.lambda_execution_role.arn
  package_type = "Image"
  image_uri    = "${var.ecr_repo.notification-by-gokabot.repository_url}:latest"
  timeout      = 300

  lifecycle {
    ignore_changes = [image_uri]
  }

  environment {
    variables = {
      LINE_CHANNEL_TOKEN = var.line_channel_token
      MY_USER_ID         = var.my_user_id
    }
  }

  tags = {
    Name = "notification-by-gokabot"
    cost = var.cost_tag
  }
}

resource "aws_lambda_permission" "notification-by-gokabot-with-sns" {
  statement_id  = "AllowExecutionFromSNS"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.notification-by-gokabot.function_name
  principal     = "sns.amazonaws.com"
  source_arn    = var.sns_topic.notification-by-gokabot.arn
}

resource "aws_lambda_function" "notification-to-msteams" {
  function_name = "notification-to-msteams"

  role         = var.lambda_execution_role.arn
  package_type = "Image"
  image_uri    = "${var.ecr_repo.notification-to-msteams.repository_url}:latest"
  timeout      = 300

  lifecycle {
    ignore_changes = [image_uri]
  }

  environment {
    variables = {
      WEBHOOK_URL = var.webhook_url
    }
  }

  tags = {
    Name = "notification-to-msteams"
    cost = var.cost_tag
  }
}

resource "aws_lambda_permission" "notification-to-msteams-with-sns" {
  statement_id  = "AllowExecutionFromSNS"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.notification-to-msteams.function_name
  principal     = "sns.amazonaws.com"
  source_arn    = var.sns_topic.notification-to-msteams.arn
}
