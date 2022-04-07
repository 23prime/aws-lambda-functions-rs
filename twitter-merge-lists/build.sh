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
docker-compose build
