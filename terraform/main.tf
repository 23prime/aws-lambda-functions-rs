terraform {
  required_version = "0.14.8"

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
  ecr_repo              = module.ecr.notification-by-gokabot-repo
  line_channel_token    = var.line_channel_token
  my_user_id            = var.my_user_id
  sns_topic             = module.sns.notification-by-gokabot-topic
}

module "sns" {
  source          = "./modules/sns"
  cost_tag        = var.cost_tag
  aws_account_id  = var.aws_account_id
  lambda_function = module.lambda.notification-by-gokabot
}
