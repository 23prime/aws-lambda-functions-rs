variable "cost_tag" {
  type = string
}

variable "ecr_repo" {
  type = object({
    notification-by-gokabot = any
    notification-to-msteams = any
    twitter-followee-list   = any
  })
}

variable "line_channel_token" {
  type = string
}

variable "my_user_id" {
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

variable "event_rules" {
  type = object({
    twitter-followee-list-schedule = any
  })
}
