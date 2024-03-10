#!/bin/bash
set -e

TEAM_NAME=$1
ACR_NAME=$2
DOMAIN_NAME=$3

IMAGE_TAG="latest"
PROJECT_NAME="TODO"
CLUSTER_NAME="team"

REPO_NAME="${ACR_NAME}.TODO"
IMAGE_NAME="TODO"

# TODO Log into the ACR
# TODO Build the image
# TODO Push the image to the ACR
# TODO Retrieve kubernetes credentials / add them to .kubeconfig
# TODO Deploy or upgrade helm charts
