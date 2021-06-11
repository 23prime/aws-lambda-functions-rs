variable "cost_tag" {
  type = string
}

variable "aws_account_id" {
  type = string
}

variable "lambda_function" {
  type = object({
    notification-by-gokabot = any
    notification-to-msteams = any
  })
}
