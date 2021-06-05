# Get KMS Key data
data "aws_kms_key" "ssm_key" {
  key_id = "alias/aws/ssm"
}

# For LambdaExecutionRoleWithGokabotSecretAccess
resource "aws_iam_policy" "GokabotSecretAccessForLambda" {
  name = "GokabotSecretAccessForLambda"
  path = "/service-role/"

  policy = jsonencode(
    {
      Version = "2012-10-17",
      Statement = [
        {
          Action = [
            "ssm:GetParameters",
            "kms:Decrypt",
          ]
          Effect = "Allow"
          Resource = [
            "arn:aws:ssm:ap-northeast-1:${var.aws_account_id}:parameter/gokabot*",
            data.aws_kms_key.ssm_key.arn
          ]
        }
      ]
    }
  )

  tags = {
    Name = "GokabotSecretAccessForLambda"
    cost = var.cost_tag
  }
}

resource "aws_iam_role" "LambdaExecutionRoleWithGokabotSecretAccess" {
  name = "LambdaExecutionRoleWithGokabotSecretAccess"
  path = "/service-role/"

  assume_role_policy = file("${path.module}/assume_role_policy_lambda.json")

  managed_policy_arns = [
    "arn:aws:iam::aws:policy/AWSLambdaExecute",
    aws_iam_policy.GokabotSecretAccessForLambda.arn
  ]

  tags = {
    Name = "LambdaExecutionRoleWithGokabotSecretAccess"
    cost = var.cost_tag
  }
}
