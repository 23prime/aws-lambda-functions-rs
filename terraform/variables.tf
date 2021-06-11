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

  default = "gokabot"
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
