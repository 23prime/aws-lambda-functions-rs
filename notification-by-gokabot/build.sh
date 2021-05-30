#!/bin/bash

# Build container for musl build
docker compose build -f docker-compose.build.yml
docker compose run -f docker-compose.build.yml --rm notification-by-gokabot-build

# Build main container
docker compose build -f docker-compose.yml

# Push
aws ecr get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin 678084882233.
docker tag notification-by-gokabot:latest 678084882233.dkr.ecr.ap-northeast-1.amazonaws.com/notification-by-gokabot:latest
dkr.ecr.ap-northeast-1.amazonaws.com
docker push 678084882233.dkr.ecr.ap-northeast-1.amazonaws.com/notification-by-gokabot:latest
