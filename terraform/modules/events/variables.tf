variable "cost_tag" {
  type = string
}

variable "lambda_function" {
  type = object({
    twitter-followee-list         = any
    twitter-merge-vtubers-lists   = any
    one-punch-man-update-checker  = any
    gokabot-random-message-caller = any
  })
}
