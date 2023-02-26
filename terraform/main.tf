terraform {
  required_version = "~>1.0"

  backend "s3" {
    bucket = "tfstate-aws-lambda-functions-rs"
    key    = "terraform.tfstate"
    region = "ap-northeast-1"
  }

  required_providers {
    aws = "3.38.0"
  }
}

provider "aws" {
  access_key = var.aws_access_key_id
  secret_key = var.aws_secret_access_key
  region     = var.aws_region
}

module "iam" {
  source         = "./modules/iam"
  cost_tag       = var.cost_tag
  aws_account_id = var.aws_account_id
}

module "ecr" {
  source   = "./modules/ecr"
  cost_tag = var.cost_tag
}

module "lambda" {
  source                = "./modules/lambda"
  cost_tag              = var.cost_tag
  lambda_execution_role = module.iam.LambdaExecutionRoleWithGokabotSecretAccess
  ecr_repo = {
    notification-by-gokabot       = module.ecr.notification-by-gokabot-repo
    notification-to-msteams       = module.ecr.notification-to-msteams-repo
    twitter-followee-list         = module.ecr.twitter-followee-list-repo
    twitter-merge-lists           = module.ecr.twitter-merge-lists-repo
    tonarinoyj-update-checker     = module.ecr.tonarinoyj-update-checker-repo
    gokabot-random-message-caller = module.ecr.gokabot-random-message-caller-repo
  }
  line_channel_token = var.line_channel_token
  my_user_id         = var.my_user_id
  nga_group_id       = var.nga_group_id
  webhook_url        = var.webhook_url
  sns_topic = {
    notification-by-gokabot = module.sns.notification-by-gokabot-topic
    notification-to-msteams = module.sns.notification-to-msteams-topic
  }
  twitter_user_id                  = var.twitter_user_id
  twitter_list_id                  = var.twitter_list_id
  twitter_source_vtubers_lists     = var.twitter_source_vtubers_lists
  twitter_target_vtubers_list      = var.twitter_target_vtubers_list
  twitter_access_token             = var.twitter_access_token
  twitter_access_token_secret      = var.twitter_access_token_secret
  twitter_consumer_key             = var.twitter_consumer_key
  twitter_consumer_secret          = var.twitter_consumer_secret
  one_punch_man_series_id          = var.one_punch_man_series_id
  tonarinoyj_update_checker_db_url = var.tonarinoyj_update_checker_db_url
  gokabot_base_uri                 = var.gokabot_base_uri
  target_ids                       = var.target_ids
  event_rules = {
    twitter-followee-list-schedule         = module.events.twitter-followee-list-schedule
    twitter-merge-vtubers-lists-schedule   = module.events.twitter-merge-vtubers-lists-schedule
    one-punch-man-update-checker-schedule  = module.events.one-punch-man-update-checker-schedule
    gokabot-random-message-caller-schedule = module.events.gokabot-random-message-caller-schedule
  }
}

module "sns" {
  source         = "./modules/sns"
  cost_tag       = var.cost_tag
  aws_account_id = var.aws_account_id
  lambda_function = {
    notification-by-gokabot = module.lambda.notification-by-gokabot
    notification-to-msteams = module.lambda.notification-to-msteams
  }
}

module "events" {
  source   = "./modules/events"
  cost_tag = var.cost_tag
  lambda_function = {
    twitter-followee-list         = module.lambda.twitter-followee-list
    twitter-merge-vtubers-lists   = module.lambda.twitter-merge-vtubers-lists
    one-punch-man-update-checker  = module.lambda.one-punch-man-update-checker
    gokabot-random-message-caller = module.lambda.gokabot-random-message-caller
  }
}
