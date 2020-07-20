#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

# https://github.com/mdirkse/rust_armv6

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
WORKDIR="/work"
IMAGE_NAME="raspberrypi-build"

docker build -t ${IMAGE_NAME} -f "$DIR/Dockerfile" .
docker run --rm -t \
       -u "$(id -u):$(id -g)" \
       -e "HOME=${WORKDIR}" \
       -e "PKG_CONFIG_ALLOW_CROSS=1" \
       -w "${WORKDIR}" \
       -v "$(pwd):${WORKDIR}" \
       -v "${HOME}/.cargo/registry:/usr/local/cargo/registry" \
       ${IMAGE_NAME}
