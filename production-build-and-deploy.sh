#!/bin/sh

# Do the building and pushing
IMAGE_ID=dt-user-service
GCR_ID=gm-production-296501

docker build --platform=linux/arm64 -t gcr.io/$GCR_ID/$IMAGE_ID:latest . --push

# # Trigger deployment to happen
# sh ./ci/cloudbuild-trigger-deploy.sh product-295902 dt-configs-backend develop
