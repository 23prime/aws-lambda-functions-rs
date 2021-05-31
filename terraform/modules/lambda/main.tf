data "aws_iam_role" "LambdaExecutionRoleWithGokabotSecretAccess" {
  name = "LambdaExecutionRoleWithGokabotSecretAccess"
}

data "aws_sns_topic" "notification-by-gokabot" {
  name = "notification-by-gokabot"
}

resource "aws_lambda_function" "notification-by-gokabot" {
  function_name = "notification-by-gokabot-rs"

  role         = data.aws_iam_role.LambdaExecutionRoleWithGokabotSecretAccess.arn
  package_type = "Image"
  image_uri    = "${var.ecr_repo.repository_url}:latest"
  timeout      = 300

  lifecycle {
    ignore_changes = [image_uri]
  }

  environment {
    variables = {
      LINE_CHANNEL_TOKEN = var.line_channel_token
      MY_USER_ID = var.my_user_id
    }
  }  

  tags = {
    Name = "notification-by-gokabot-rs"
    cost = var.cost_tag
  }
}
