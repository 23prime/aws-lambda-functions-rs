variable "aws_account_id" {
  type = string
}

variable "aws_access_key_id" {
  type = string
}

variable "aws_secret_access_key" {
  type = string
}

variable "aws_region" {
  type = string
}

variable "cost_tag" {
  type = string

  default = "lambda-tools"
}

variable "line_channel_token" {
  type = string
}

variable "my_user_id" {
  type = string
}

variable "nga_group_id" {
  type = string
}

variable "webhook_url" {
  type = string
}

variable "twitter_user_id" {
  type = string
}

variable "twitter_list_id" {
  type = string
}

variable "twitter_source_vtubers_lists" {
  type = string
}

variable "twitter_target_vtubers_list" {
  type = string
}

variable "twitter_access_token" {
  type = string
}

variable "twitter_access_token_secret" {
  type = string
}

variable "twitter_consumer_key" {
  type = string
}

variable "twitter_consumer_secret" {
  type = string
}

variable "one_punch_man_series_id" {
  type = string
}

variable "tonarinoyj_update_checker_db_url" {
  type = string
}

variable "gokabot_base_uri" {
  type = string
}
