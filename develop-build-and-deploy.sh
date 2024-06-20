#!/bin/sh

# Do the building and pushing
IMAGE_ID=dt-user-service
GCR_ID=gm-develop

docker build -f ./local.dockerfile -t gcr.io/$GCR_ID/$IMAGE_ID:latest .
docker push gcr.io/$GCR_ID/$IMAGE_ID

# # Trigger deployment to happen
# sh ./ci/cloudbuild-trigger-deploy.sh product-295902 dt-configs-backend develop
