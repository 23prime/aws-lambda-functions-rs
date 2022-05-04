#!/bin/bash -eu

# Set variables
DOCKER_BUILD_CONTAINER_NAME=tonarinoyj-update-checker-build

# Build container for musl build
docker-compose -f docker-compose.build.yml build
docker-compose -f docker-compose.build.yml run --rm $DOCKER_BUILD_CONTAINER_NAME

# Build main container
docker-compose build
