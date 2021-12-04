#!/bin/bash

# Build container for musl build
docker compose build -f docker-compose.build.yml
docker compose run -f docker-compose.build.yml --rm twitter-followee-list-build

# Build main container
docker compose build -f docker-compose.yml

# Push
aws ecr get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin 678084882233.dkr.ecr.ap-northeast-1.amazonaws.com
docker tag twitter-followee-list:latest 678084882233.dkr.ecr.ap-northeast-1.amazonaws.com/twitter-followee-list:latest
docker push 678084882233.dkr.ecr.ap-northeast-1.amazonaws.com/twitter-followee-list:latest
