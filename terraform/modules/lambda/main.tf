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

resource "aws_lambda_function" "twitter-followee-list" {
  function_name = "twitter-followee-list"

  role         = var.lambda_execution_role.arn
  package_type = "Image"
  image_uri    = "${var.ecr_repo.twitter-followee-list.repository_url}:latest"
  timeout      = 300

  lifecycle {
    ignore_changes = [image_uri]
  }

  environment {
    variables = {
      USER_ID             = var.twitter_user_id
      LIST_ID             = var.twitter_list_id
      ACCESS_TOKEN        = var.twitter_access_token
      ACCESS_TOKEN_SECRET = var.twitter_access_token_secret
      CONSUMER_KEY        = var.twitter_consumer_key
      CONSUMER_SECRET     = var.twitter_consumer_secret
    }
  }

  tags = {
    Name = "twitter-followee-list"
    cost = var.cost_tag
  }
}

resource "aws_lambda_permission" "twitter-followee-list-with-event" {
  statement_id  = "AllowExecutionFromCloudWatch"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.twitter-followee-list.function_name
  principal     = "events.amazonaws.com"
  source_arn    = var.event_rules.twitter-followee-list-schedule.arn
}

resource "aws_lambda_function" "twitter-merge-vtubers-lists" {
  function_name = "twitter-merge-vtubers-lists"

  role         = var.lambda_execution_role.arn
  package_type = "Image"
  image_uri    = "${var.ecr_repo.twitter-merge-lists.repository_url}:latest"
  timeout      = 300

  lifecycle {
    ignore_changes = [image_uri]
  }

  environment {
    variables = {
      SOURCE_LISTS        = var.twitter_source_vtubers_lists
      TARGET_LIST         = var.twitter_target_vtubers_list
      ACCESS_TOKEN        = var.twitter_access_token
      ACCESS_TOKEN_SECRET = var.twitter_access_token_secret
      CONSUMER_KEY        = var.twitter_consumer_key
      CONSUMER_SECRET     = var.twitter_consumer_secret
    }
  }

  tags = {
    Name = "twitter-merge-vtubers-lists"
    cost = var.cost_tag
  }
}

resource "aws_lambda_permission" "twitter-merge-vtubers-lists-with-event" {
  statement_id  = "AllowExecutionFromCloudWatch"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.twitter-merge-vtubers-lists.function_name
  principal     = "events.amazonaws.com"
  source_arn    = var.event_rules.twitter-merge-vtubers-lists-schedule.arn
}

resource "aws_lambda_function" "one-punch-man-update-checker" {
  function_name = "one-punch-man-update-checker"

  role         = var.lambda_execution_role.arn
  package_type = "Image"
  image_uri    = "${var.ecr_repo.tonarinoyj-update-checker.repository_url}:latest"
  timeout      = 300

  lifecycle {
    ignore_changes = [image_uri]
  }

  environment {
    variables = {
      TYJ_SERIES_ID      = var.one_punch_man_series_id
      DATABASE_URL       = var.tonarinoyj_update_checker_db_url
      LINE_CHANNEL_TOKEN = var.line_channel_token
      TARGET_ID          = var.nga_group_id
    }
  }

  tags = {
    Name = "one-punch-man-update-checker"
    cost = var.cost_tag
  }
}

resource "aws_lambda_permission" "one-punch-man-update-checker-with-event" {
  statement_id  = "AllowExecutionFromCloudWatch"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.one-punch-man-update-checker.function_name
  principal     = "events.amazonaws.com"
  source_arn    = var.event_rules.one-punch-man-update-checker-schedule.arn
}

resource "aws_lambda_function" "gokabot-random-message-caller" {
  function_name = "gokabot-random-message-caller"

  role         = var.lambda_execution_role.arn
  package_type = "Image"
  image_uri    = "${var.ecr_repo.gokabot-random-message-caller.repository_url}:latest"
  timeout      = 300

  lifecycle {
    ignore_changes = [image_uri]
  }

  environment {
    variables = {
      GOKABOT_BASE_URI = var.gokabot_base_uri
      TARGET_IDS       = var.target_ids
    }
  }

  tags = {
    Name = "gokabot-random-message-caller"
    cost = var.cost_tag
  }
}

resource "aws_lambda_permission" "gokabot-random-message-caller-with-event" {
  statement_id  = "AllowExecutionFromCloudWatch"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.gokabot-random-message-caller.function_name
  principal     = "events.amazonaws.com"
  source_arn    = var.event_rules.gokabot-random-message-caller-schedule.arn
}
