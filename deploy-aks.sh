#!/bin/bash
set -e

TEAM_NAME=$1
ACR_NAME=$2
DOMAIN_NAME=$3

IMAGE_TAG="latest"
PROJECT_NAME="rusters"
CLUSTER_NAME="team"

REPO_NAME="${ACR_NAME}.azurecr.io"
IMAGE_NAME="${REPO_NAME}/${PROJECT_NAME}:${IMAGE_TAG}"

# TODO Log into the ACR
# TODO Build the image
# TODO Push the image to the ACR
# TODO Retrieve kubernetes credentials / add them to .kubeconfig

VARIABLES=... # TOTO set variables for helm deployment

if ! (helm ls  | grep $PROJECT_NAME) then
   echo "Installing Helm Chart"
   eval helm install -f helm/values.yaml $PROJECT_NAME helm/ $VARIABLES
else
   echo "Upgrading Helm Chart"
   eval helm upgrade -f helm/values.yaml $PROJECT_NAME helm/ $VARIABLES
fi
