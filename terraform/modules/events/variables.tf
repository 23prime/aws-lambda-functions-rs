variable "cost_tag" {
  type = string
}

variable "lambda_function" {
  type = object({
    twitter-followee-list = any
  })
}
