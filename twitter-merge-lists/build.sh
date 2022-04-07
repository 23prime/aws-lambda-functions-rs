#!/bin/bash -eu

# Set variables
DOCKER_IMAGE_NAME=twitter-merge-lists
DOCKER_BUILD_CONTAINER_NAME=twitter-merge-lists-build

# Build container for musl build
docker-compose -f docker-compose.build.yml build
docker-compose -f docker-compose.build.yml run --rm $DOCKER_BUILD_CONTAINER_NAME

# Build main container
docker-compose build
