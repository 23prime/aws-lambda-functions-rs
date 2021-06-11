variable "cost_tag" {
  type = string
}

variable "ecr_repo" {
  type = object({
    notification-by-gokabot = any
    notification-to-msteams = any
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
