#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

# https://github.com/mdirkse/rust_armv6

DIRECTORY=$(cd `dirname $0` && pwd)
WORKDIR="/work"
IMAGE_NAME="raspberrypi-build"

docker build -t ${IMAGE_NAME} -f "$DIRECTORY/Dockerfile" .
docker run --rm -t \
       -e "HOME=${WORKDIR}" \
       -e "PKG_CONFIG_ALLOW_CROSS=1" \
       -w "${WORKDIR}" \
       -v "${DIRECTORY}/..:${WORKDIR}" \
       -v "${HOME}/.cargo/registry:/usr/local/cargo/registry" \
       ${IMAGE_NAME}
