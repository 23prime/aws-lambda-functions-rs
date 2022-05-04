#!/bin/bash -eu

# Set variables
DOCKER_IMAGE_NAME=tonarinoyj-update-checker
AWS_ACCOUNT_ID=678084882233
AWS_REGION=ap-northeast-1

# Push
aws ecr get-login-password --region $AWS_REGION | docker login --username AWS --password-stdin $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com
docker tag $DOCKER_IMAGE_NAME:latest $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/$DOCKER_IMAGE_NAME:latest
docker push $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/$DOCKER_IMAGE_NAME:latest

# TODO: Deploy to Lambda from ECR
