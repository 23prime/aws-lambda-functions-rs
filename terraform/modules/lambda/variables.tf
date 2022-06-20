variable "cost_tag" {
  type = string
}

variable "ecr_repo" {
  type = object({
    notification-by-gokabot       = any
    notification-to-msteams       = any
    twitter-followee-list         = any
    twitter-merge-lists           = any
    tonarinoyj-update-checker     = any
    gokabot-random-message-caller = any
  })
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

variable "sns_topic" {
  type = object({
    notification-by-gokabot = any
    notification-to-msteams = any
  })
}

variable "lambda_execution_role" {
  type = any
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

variable "target_ids" {
  type = string
}

variable "event_rules" {
  type = object({
    twitter-followee-list-schedule         = any
    twitter-merge-vtubers-lists-schedule   = any
    one-punch-man-update-checker-schedule  = any
    gokabot-random-message-caller-schedule = any
  })
}
