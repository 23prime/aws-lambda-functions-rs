terraform {
  required_version = "1.0.3"

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
    notification-by-gokabot = module.ecr.notification-by-gokabot-repo
    notification-to-msteams = module.ecr.notification-to-msteams-repo
    twitter-followee-list   = module.ecr.twitter-followee-list-repo
  }
  line_channel_token = var.line_channel_token
  my_user_id         = var.my_user_id
  webhook_url        = var.webhook_url
  sns_topic = {
    notification-by-gokabot = module.sns.notification-by-gokabot-topic
    notification-to-msteams = module.sns.notification-to-msteams-topic
  }
  twitter_user_id             = var.twitter_user_id
  twitter_list_id             = var.twitter_list_id
  twitter_access_token        = var.twitter_access_token
  twitter_access_token_secret = var.twitter_access_token_secret
  twitter_consumer_key        = var.twitter_consumer_key
  twitter_consumer_secret     = var.twitter_consumer_secret
  event_rules = {
    twitter-followee-list-schedule = module.events.twitter-followee-list-schedule
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
    twitter-followee-list = module.lambda.twitter-followee-list
  }
}
