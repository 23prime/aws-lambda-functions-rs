#!/bin/bash -eu

# Set variables
DOCKER_IMAGE_NAME=twitter-merge-lists
DOCKER_BUILD_CONTAINER_NAME=twitter-merge-lists-build
AWS_ACCOUNT_ID=678084882233
AWS_REGION=ap-northeast-1

# Build container for musl build
docker-compose -f docker-compose.build.yml build
docker-compose -f docker-compose.build.yml run --rm $DOCKER_BUILD_CONTAINER_NAME

# Build main container
docker-compose -f docker-compose.yml build

# Push
aws ecr get-login-password --region $AWS_REGION | docker login --username AWS --password-stdin $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com
docker tag $DOCKER_IMAGE_NAME:latest $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/$DOCKER_IMAGE_NAME:latest
docker push $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/$DOCKER_IMAGE_NAME:latest
